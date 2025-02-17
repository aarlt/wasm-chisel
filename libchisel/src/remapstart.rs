use parity_wasm::elements::*;

use super::{ModuleError, ModulePreset, ModuleTranslator};

pub struct RemapStart;

impl ModulePreset for RemapStart {
    fn with_preset(preset: &str) -> Result<Self, ()> {
        match preset {
            // TODO refactor this later
            "ewasm" => Ok(RemapStart {}),
            _ => Err(()),
        }
    }
}

impl ModuleTranslator for RemapStart {
    fn translate_inplace(&self, module: &mut Module) -> Result<bool, ModuleError> {
        Ok(remap_start(module))
    }

    fn translate(&self, module: &Module) -> Result<Option<Module>, ModuleError> {
        let mut ret = module.clone();
        if remap_start(&mut ret) {
            Ok(Some(ret))
        } else {
            Ok(None)
        }
    }
}

// NOTE: This seems to be exported properly in later versions of parity-wasm.
// TODO: When updated, use the proper method instead.
fn section_order(s: &Section) -> u8 {
    match s {
        Section::Custom(_) => 0x00,
        Section::Unparsed { .. } => 0x00,
        Section::Type(_) => 0x1,
        Section::Import(_) => 0x2,
        Section::Function(_) => 0x3,
        Section::Table(_) => 0x4,
        Section::Memory(_) => 0x5,
        Section::Global(_) => 0x6,
        Section::Export(_) => 0x7,
        Section::Start(_) => 0x8,
        Section::Element(_) => 0x9,
        Section::Code(_) => 0x0a,
        Section::Data(_) => 0x0b,
        Section::Name(_) => 0x00,
        Section::Reloc(_) => 0x00,
    }
}

/// Replace an exported function with another function, or export if unexported.
fn remap_or_export_main(module: &mut Module, export_name: &str, func_idx: u32) {
    let new_func_export = ExportEntry::new(export_name.to_string(), Internal::Function(func_idx));

    if let Some(export_section) = module.export_section_mut() {
        let export_section = export_section.entries_mut();
        // If we find an export named `export_name`, replace it. Otherwise, append an entry to the
        // section with the supplied func index.
        if let Some(main_export_loc) = export_section
            .iter_mut()
            .position(|e| *e.field() == export_name.to_string())
        {
            export_section[main_export_loc] = new_func_export;
        } else {
            export_section.push(new_func_export);
        }
    } else {
        let sections = module.sections_mut();
        let new_export_section =
            Section::Export(ExportSection::with_entries(vec![new_func_export]));

        // If we can find a section that is supposed to be after exports, insert the new section at its position (shifts other elements to the right).
        // Otherwise, append it at the end.
        // NOTE: Assumes that the ordering of sections is otherwise correct.
        if let Some(exports_loc) = sections.iter().position(|sec| section_order(&sec) > 0x7) {
            sections.insert(exports_loc, new_export_section);
        } else {
            sections.push(new_export_section);
        }
    }
}

fn remap_start(module: &mut Module) -> bool {
    if let Some(start_func_idx) = module.start_section() {
        // Look for an export "main". If found, replace it with an export of the function to
        // which the start section points.
        remap_or_export_main(module, "main", start_func_idx);

        // Remove the start section, leaving the "main" export as the entry point.
        module.clear_start_section();

        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::SerializationHelpers;
    use crate::{ModulePreset, ModuleTranslator};
    use rustc_hex::FromHex;

    #[test]
    fn remapstart_mutation() {
        //wat:
        //(module
        //    (import "env" "ethereum_useGas" (func (param i64)))
        //    (memory 1)
        //    (export "main" (func $main))
        //    (export "memory" (memory 0))
        //    (func $main2)
        //    (func $main)
        //    (start $main2)
        //)

        let wasm: Vec<u8> = FromHex::from_hex(
            "0061736d0100000001080260017e0060
000002170103656e760f657468657265756d5f75736547617300000303020101050301000107110
2046d61696e0001066d656d6f727902000801020a070202000b02000b0020046e616d65010e0201
046d61696e02056d61696e320209030001000001000200",
        )
        .unwrap();

        let mut module = Module::from_slice(&wasm);
        module = module.parse_names().unwrap();
        assert!(module.names_section().is_some());
        let start_idx = module
            .start_section()
            .expect("Module missing start function");

        let new = RemapStart::with_preset("ewasm")
            .unwrap()
            .translate(&module)
            .expect("Module internal error")
            .expect("new module not returned");

        assert!(
            new.start_section().is_none(),
            "start section wasn't removed"
        );
        assert!(new
            .export_section()
            .expect("Module missing export section")
            .entries()
            .iter()
            .find(|e| e.field() == String::from("main")
                && *e.internal() == Internal::Function(start_idx))
            .is_some());
    }

    #[test]
    fn remapstart_no_mutation() {
        // (module
        //    (import "env" "ethereum_useGas" (func (param i64)))
        //    (memory 1)
        //    (export "main" (func $main))
        //    (export "memory" (memory 0))
        //    (func $main)
        //)

        let wasm: Vec<u8> = FromHex::from_hex(
            "0061736d0100000001080260017e0060
        000002170103656e760f657468657265756d5f757365476173000003020101050301000
        1071102046d61696e0001066d656d6f727902000a040102000b",
        )
        .unwrap();

        let module = Module::from_slice(&wasm);
        let new = RemapStart::with_preset("ewasm")
            .unwrap()
            .translate(&module)
            .expect("Module internal error");

        assert!(new.is_none());
    }

    #[test]
    fn remapstart_inplace_mutation() {
        //wat:
        //(module
        //    (import "env" "ethereum_useGas" (func (param i64)))
        //    (memory 1)
        //    (export "main" (func $main))
        //    (export "memory" (memory 0))
        //    (func $main2)
        //    (func $main)
        //    (start $main2)
        //)

        let wasm: Vec<u8> = FromHex::from_hex(
            "0061736d0100000001080260017e0060
000002170103656e760f657468657265756d5f75736547617300000303020101050301000107110
2046d61696e0001066d656d6f727902000801020a070202000b02000b0020046e616d65010e0201
046d61696e02056d61696e320209030001000001000200",
        )
        .unwrap();

        let mut module = Module::from_slice(&wasm);
        module = module.parse_names().unwrap();
        assert!(module.names_section().is_some());

        let res = RemapStart::with_preset("ewasm")
            .unwrap()
            .translate_inplace(&mut module)
            .unwrap();

        assert!(res, "module was not modified");
        assert!(
            module.start_section().is_none(),
            "start section wasn't removed"
        );
    }

    #[test]
    fn remapstart_inplace_no_mutation() {
        // (module
        //    (import "env" "ethereum_useGas" (func (param i64)))
        //    (memory 1)
        //    (export "main" (func $main))
        //    (export "memory" (memory 0))
        //    (func $main)
        //)

        let wasm: Vec<u8> = FromHex::from_hex(
            "0061736d0100000001080260017e0060
000002170103656e760f657468657265756d5f75736547617300000302010105030100010711020
46d61696e0001066d656d6f727902000a040102000b",
        )
        .unwrap();

        let mut module = Module::from_slice(&wasm);
        let res = RemapStart::with_preset("ewasm")
            .unwrap()
            .translate_inplace(&mut module)
            .unwrap();

        assert!(!res, "module was modified");
    }

    #[test]
    fn remapstart_mutation_no_exports() {
        //wat:
        //(module
        //    (import "env" "ethereum_useGas" (func (param i64)))
        //    (memory 1)
        //    (func $main2)
        //    (func $main)
        //    (start $main2)
        //)

        let wasm: Vec<u8> = FromHex::from_hex(
            "0061736d0100000001080260017e0060000002170103656e760f657468657265756d5f7573654761730000030302010105030100010801010a070202000b02000b",
        )
        .unwrap();

        let mut module = Module::from_slice(&wasm);
        let res = RemapStart::with_preset("ewasm")
            .unwrap()
            .translate_inplace(&mut module)
            .unwrap();

        assert!(res, "module was not modified");
        assert!(
            module.export_section().is_some(),
            "export section does not exist"
        );
    }

    #[test]
    fn export_section_exists_but_no_main() {
        // wat:
        // (module
        //     (import "env" "ethereum_useGas" (func (param i64)))
        //     (memory 1)
        //     (start $main)
        //     (export "memory" (memory 0))
        //     (func $main)
        // )
        let wasm: Vec<u8> = FromHex::from_hex(
            "0061736d0100000001080260017e0060000002170103656e760f657468657265756d5f7573654761730000030201010503010001070a01066d656d6f727902000801010a040102000b"
        ).unwrap();
        let mut module = Module::from_slice(&wasm);
        let remapper = RemapStart::with_preset("ewasm").expect("Can't fail");

        let res = remapper.translate_inplace(&mut module);
        assert!(res.is_ok());
        let mutated = res.unwrap();
        assert_eq!(mutated, true);
        assert!(module.export_section().is_some());
        assert!(module.start_section().is_none());
        assert!(module
            .export_section()
            .unwrap()
            .entries()
            .iter()
            .find(|e| e.field() == "main")
            .is_some());
    }
}
