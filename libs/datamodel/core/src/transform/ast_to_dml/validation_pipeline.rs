use super::*;
use crate::{ast, configuration, dml, messages::MessageCollection};

/// Is responsible for loading and validating the Datamodel defined in an AST.
/// Wrapper for all lift and validation steps
pub struct ValidationPipeline<'a> {
    lifter: LiftAstToDml<'a>,
    validator: Validator<'a>,
    standardiser: Standardiser,
}

impl<'a> ValidationPipeline<'a> {
    pub fn new(sources: &'a [configuration::Datasource]) -> ValidationPipeline<'a> {
        let source = sources.first();
        ValidationPipeline {
            lifter: LiftAstToDml::new(source),
            validator: Validator::new(source),
            standardiser: Standardiser::new(),
        }
    }

    /// Validates an AST semantically and promotes it to a datamodel/schema.
    ///
    /// This method will attempt to
    /// * Resolve all attributes
    /// * Recursively evaluate all functions
    /// * Perform string interpolation
    /// * Resolve and check default values
    /// * Resolve and check all field types
    pub fn validate(&self, ast_schema: &ast::SchemaAst) -> Result<dml::Datamodel, MessageCollection> {
        let mut all_messages = MessageCollection::new();

        // Phase 0 is parsing.
        // Phase 1 is source block loading.

        // Phase 2: Prechecks.
        if let Err(mut err) = precheck::Precheck::precheck(&ast_schema) {
            all_messages.append(&mut err);
        }

        // Early return so that the validator does not have to deal with invalid schemas
        if all_messages.has_errors() {
            return Err(all_messages);
        }

        // Phase 3: Lift AST to DML.
        let mut schema = match self.lifter.lift(ast_schema) {
            Err(mut err) => {
                // Cannot continue on lifter error.
                all_messages.append(&mut err);
                return Err(all_messages);
            }
            Ok(schema) => schema,
        };

        // Phase 4: Validation
        if let Err(mut err) = self.validator.validate(ast_schema, &mut schema) {
            all_messages.append(&mut err);
        }

        // Early return so that the standardiser does not have to deal with invalid schemas
        if all_messages.has_errors() {
            return Err(all_messages);
        }

        // TODO: Move consistency stuff into different module.
        // Phase 5: Consistency fixes. These don't fail.
        if let Err(mut err) = self.standardiser.standardise(ast_schema, &mut schema) {
            all_messages.append(&mut err);
        }

        // Early return so that the post validation does not have to deal with invalid schemas
        if all_messages.has_errors() {
            return Err(all_messages);
        }

        // Phase 6: Post Standardisation Validation
        if let Err(mut err) = self.validator.post_standardisation_validate(ast_schema, &mut schema) {
            all_messages.append(&mut err);
        }

        if all_messages.has_errors() {
            Err(all_messages)
        } else {
            Ok(schema)
        }
    }
}
