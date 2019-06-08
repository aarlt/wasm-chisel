use std::error::Error;

use libchisel::ModuleCreator;
use libchisel::ModuleError;
use libchisel::ModuleTranslator;
use libchisel::ModuleValidator;

use parity_wasm::elements::Module;

/// A set of "Runners" to be executed. The instantiated form of a chisel configuration, with an
/// arbitrary number of rulesets.
pub struct ChiselDriver<'a>(Vec<ChiselRunner<'a>>);

/// The instantiated form of a chisel ruleset.
pub enum ChiselRunner<'a> {
    /// Normal mode. Accepts a module and runs a set of validators and verifiers on it.
    NormalMode(Module, ChiselModuleSet<'a>),
    /// This variant is used when a creator module is present, and runs the translator/validator
    /// modules post-creation.
    CreateMode(&'a dyn ModuleCreator, ChiselModuleSet<'a>),
}

/// A list of Chisel modules to run. Translators are run before the validators by default.
pub struct ChiselModuleSet<'a> {
    /// Whether to instantiate in place by default.
    in_place: bool,
    /// Translators to be run. Translators are run before validators by default.
    translators: Vec<&'a dyn ModuleTranslator>,
    /// Validators to be run.
    validators: Vec<&'a dyn ModuleValidator>,
}

impl<'a> ChiselRunner<'a> {
    /// Instantiate a ruleset which operates on a module passed by the caller, and does not use
    /// creator modules.
    pub fn normal_mode(module: Module, chisel_mods: ChiselModuleSet<'a>) -> Self {
        ChiselRunner::NormalMode(module, chisel_mods)
    }

    /// Instantiate a ruleset which creates a module, rather than accepting one from the caller.
    pub fn create_mode(creator: &'a dyn ModuleCreator, chisel_mods: ChiselModuleSet<'a>) -> Self {
        ChiselRunner::CreateMode(creator, chisel_mods)
    }

    // FIXME: needs refactor
    /// Run all rulesets and propagate errors.
    pub fn run(&mut self) -> Result<(), ()> {
        match self {
            //TODO: why does only this variant need the ref pattern?
            ChiselRunner::NormalMode(ref mut module, ref mut chisel_mods) => {
                let translation_results = chisel_mods.run_translators(module);
                // handle errors
                let validation_results = chisel_mods.run_validators(module);
                //TODO
                Ok(())
            }
            ChiselRunner::CreateMode(creator, chisel_mods) => {
                // TODO: Need to propagate errors here when it is implemented.
                let mut module = creator.create().expect("creation failed");

                // Run all the translators first.
                let translation_results = chisel_mods.run_translators(&mut module);

                // If any of the modules had an unrecoverable error, then propagate it.
                if let Some(e) = translation_results.iter().find(|result| match result {
                    Err(_) => true,
                    Ok(_) => false,
                }) {
                    //TODO: proper error handling
                    panic!("Translator module internal error");
                }

                // Validate.
                let validation_results = chisel_mods.run_validators(&mut module);

                // Propagate any errors here.
                if let Some(e) = validation_results.iter().find(|result| match result {
                    Err(_) => true,
                    Ok(_) => false,
                }) {
                    //TODO: proper error handling
                    panic!("Validator module internal error");
                }

                Ok(())
            }
        }
    }
}

impl<'a> ChiselModuleSet<'a> {
    /// Construct an empty module set, defaulting not to translate in place.
    pub fn new() -> Self {
        ChiselModuleSet {
            in_place: false,
            translators: Vec::new(),
            validators: Vec::new(),
        }
    }

    /// Directly instantiate from arguments.
    pub fn with_modules(
        translate_in_place: bool,
        translator_mods: Vec<&'a dyn ModuleTranslator>,
        validator_mods: Vec<&'a dyn ModuleValidator>,
    ) -> Self {
        ChiselModuleSet {
            in_place: translate_in_place,
            translators: translator_mods,
            validators: validator_mods,
        }
    }

    /// Take any type implementing ModuleTranslator and add it to the set.
    pub fn add_translator<T: ModuleTranslator>(&mut self, translator: &'a T) {
        self.translators.push(translator as &dyn ModuleTranslator);
    }

    /// Take any type implementing ModuleValidator and add it to the set.
    pub fn add_validator<V: ModuleValidator>(&mut self, validator: &'a V) {
        self.validators.push(validator as &dyn ModuleValidator);
    }

    /// Runs all the translators in the module set. On success, returns a Vec of results describing
    /// the status of the contained chisel modules' executions.
    pub fn run_translators(&mut self, mut module: &mut Module) -> Vec<Result<bool, ModuleError>> {
        let mut ret: Vec<Result<bool, ModuleError>> = Vec::new();
        // Execute each chisel module and push the results into the return buffer.
        for chisel_mod in self.translators.iter() {
            // If configured to translate in place, then attempt to and fallback to conventional
            // translation (optionally).
            if self.in_place {
                match chisel_mod.translate_inplace(&mut module) {
                    Ok(mutated) => ret.push(Ok(mutated)),
                    Err(e) => {
                        if e == ModuleError::NotSupported {
                            // If we allow to fallback to functional mode, try doing so
                        }
                    }
                }
            } else {
                // Translate conventionally.
                match chisel_mod.translate(&module) {
                    Ok(m) => {
                        if let Some(mutated) = m {
                            // Set the module to the returned translation.
                            *module = mutated;
                            ret.push(Ok(true));
                        } else {
                            ret.push(Ok(false));
                        }
                    }
                    Err(e) => ret.push(Err(e)),
                }
            }
        }
        ret
    }

    pub fn run_validators(&self, module: &Module) -> Vec<Result<bool, ModuleError>> {
        // Return type of ModuleValidator is the same type, so simply iterate over the provided
        // modules and map them to their results.
        self.validators
            .iter()
            .map(|m| m.validate(&module))
            .collect()
    }
}
