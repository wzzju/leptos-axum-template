use crate::app::AppRoutes;
use crate::components::*;
use cfg_if::cfg_if;
use leptonic::components::prelude::*;
use leptos::*;
use log::error;
use thiserror::Error;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos_actix::ResponseOptions;
        use actix_web::http::StatusCode;
    }
}

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    #[cfg(feature = "ssr")]
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorPage(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => create_rw_signal(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    error!("Errors: {errors:#?}");

    let num_errors = errors.len();

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    cfg_if! {
        if #[cfg(feature = "ssr")] {
            let response = use_context::<ResponseOptions>();
            if let Some(response) = response {
                response.set_status(errors[0].status_code());
            }
        }
    }
    let children = move |(_index, error)| match error {
        AppError::NotFound => {
            cfg_if! {
                if #[cfg(feature = "ssr")] {
                    let err_msg = error.status_code().to_string();
                } else {
                    let err_msg = error.to_string();
                }
            }
            view! { <P id="whoops">{err_msg}</P> }
        }
    };

    view! {
        <PageTitle text="Not Found"/>

        <Box class="container flex items-center flex-col err-page">
            <H1 id="error">
                {match num_errors {
                    1 => "Error",
                    _ => "Errors",
                }}

            </H1>

            <For
                each=move || { errors.clone().into_iter().enumerate() }
                key=|(index, _error)| *index
                children=children
            />

            <LinkButton id="back-btn" href=AppRoutes::Home>
                "Back Home"
            </LinkButton>
        </Box>
    }
}
