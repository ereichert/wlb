use hbs::{Handlebars, RenderError, RenderContext, Helper, Context, Renderable};

const FACTOR_OF_INTEREST_IDX: usize = 0;
const CANDIDATE_IDX: usize = 1;
pub fn if_multiple_of_helper(ctx: &Context, helper: &Helper, hbars: &Handlebars, render_ctx: &mut RenderContext) -> Result<(), RenderError> {
    let factor_of_interest = try!(
        helper.param(FACTOR_OF_INTEREST_IDX)
            .map(|json| json.value())
            .and_then(|val| val.as_u64())
            .and_then(|u64_val| if u64_val > 0 { Some(u64_val) } else { None } )
            .ok_or_else(|| RenderError::new("Factor of interest must be a number greater than 0."))
    );

    let candidate = try!(
        helper.param(CANDIDATE_IDX)
            .map(|json| json.value())
            .and_then(|val| val.as_u64())
            .ok_or_else(|| RenderError::new("Candidate must be a number greater than or equal to 0."))
    );

    let possible_template = if candidate % factor_of_interest == 0 {
        helper.template()
    } else {
        helper.inverse()
    };

    match possible_template {
        Some(t) => t.render(ctx, hbars, render_ctx),
        None => Ok(()),
    }
}

#[cfg(test)]
mod if_multiple_of_helper_tests {
    extern crate handlebars as hbs;

    use hbs::{Handlebars, Template};

    #[derive(ToJson)]
    struct TestCandidate {
        value: i64
    }

    #[test]
    fn should_return_an_err_when_one_of_the_parameters_is_missing() {
        let template = Template::compile(
            "{{#if-multiple-of 2}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 2});

        assert!(rendered.is_err());
    }

    #[test]
    fn should_return_an_err_when_the_candidate_is_less_than_0() {
        let template = Template::compile(
            "{{#if-multiple-of 2 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: -3});

        assert!(rendered.is_err());
    }

    #[test]
    fn should_return_an_err_when_the_factor_of_interest_is_0() {
        let template = Template::compile(
            "{{#if-multiple-of 0 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 3});

        assert!(rendered.is_err());
    }

    #[test]
    fn should_return_an_err_when_the_factor_of_interest_is_less_than_0() {
        let template = Template::compile(
            "{{#if-multiple-of -1 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 3});

        assert!(rendered.is_err());
    }

    #[test]
    fn should_return_the_is_not_multiple_template_when_the_candidate_is_a_multiple_of_the_factor() {
        let template = Template::compile(
            "{{#if-multiple-of 2 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 3});

        assert_eq!("IS_NOT_MULTIPLE", rendered.ok().unwrap());
    }

    #[test]
    fn should_return_the_is_multiple_template_when_the_candidate_is_a_multiple_of_the_factor() {
        let template = Template::compile(
            "{{#if-multiple-of 2 this.value}}\
                IS_MULTIPLE\
            {{else}}\
                IS_NOT_MULTIPLE\
            {{/if-multiple-of}}".to_string());

        let mut hbs = Handlebars::new();
        hbs.register_template("template", template.unwrap());
        hbs.register_helper("if-multiple-of", Box::new(super::if_multiple_of_helper));

        let rendered = hbs.render("template", &TestCandidate { value: 2});

        assert_eq!("IS_MULTIPLE", rendered.ok().unwrap());
    }
}