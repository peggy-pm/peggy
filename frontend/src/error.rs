use yew::prelude::*;

#[derive(Debug)]
pub struct ErrorPromptProps {
    // this field isnt actually used
    // instead, the `err` Method on `ErrorPromptPropsBuilder` will be called. There is however a
    // closure in the generated macro code in the html! macro that ensures this field is public
    //
    // store error only in case we want do provide some backtrace in the future
    pub err: anyhow::Error,
    msg: String,
}

impl PartialEq for ErrorPromptProps {
    fn eq(&self, other: &Self) -> bool {
        self.msg == other.msg
    }
}

impl Properties for ErrorPromptProps {
    type Builder = ErrorPromptPropsBuilder;

    fn builder() -> Self::Builder {
        Default::default()
    }
}

#[derive(Default)]
pub struct ErrorPromptPropsBuilder(Option<ErrorPromptProps>);

impl ErrorPromptPropsBuilder {
    pub fn err(self, err: impl Into<anyhow::Error>) -> Self {
        let err = err.into();
        Self(Some(ErrorPromptProps {
            msg: err.to_string(),
            err,
        }))
    }

    pub fn build(self) -> ErrorPromptProps {
        self.0.unwrap()
    }
}

pub struct ErrorPrompt;

impl Component for ErrorPrompt {
    type Message = ();
    type Properties = ErrorPromptProps;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // TODO: add some good layout
        html! {
            <div>
                <h1>{ "Ooops" }</h1>
                
                <p>{ format!("Something went wrong: {}", ctx.props().err) }</p>
            </div>
        }
    }
}
