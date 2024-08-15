use leptos::{create_effect, RwSignal, ServerFnError};
use tracing::error;

use crate::components::alert_message::{AlertMessages, MessageOptions, MessageVariant};

pub fn wrap_action_value<Response, ResponseError>(
    alert: AlertMessages,
    action_value: RwSignal<Option<Result<Result<Response, ResponseError>, ServerFnError>>>,
    function: impl Fn(Result<Response, ResponseError>) + 'static,
) where
    Response: Clone,
    ResponseError: Clone,
{
    create_effect(move |_| match action_value() {
        Some(val) => {
            match val {
                Ok(val) => function(val),
                Err(err) => match err {
                    ServerFnError::Request(err) => {
                        error!(title = "Server function error", error = err);
                        alert.create("The connection to the server could not be established. Please try again later.", MessageVariant::Failure, Default::default());
                    }
                    err => {
                        error!(title = "Server function error", error = ?err);
                        alert.create("An error occurred while processing the request. Please try again later.", MessageVariant::Failure, MessageOptions {
                            description: Some(format!("{err:?}")),
                            ..Default::default()
                        });
                    }
                },
            }
        }
        None => (),
    });
}
