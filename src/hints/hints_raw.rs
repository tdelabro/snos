pub const STARKNET_OS_INPUT: &str =
    "from starkware.starknet.core.os.os_input import StarknetOsInput\n\nos_input = \
     StarknetOsInput.load(data=program_input)\n\nids.initial_carried_outputs.messages_to_l1 = \
     segments.add_temp_segment()\nids.initial_carried_outputs.messages_to_l2 = segments.add_temp_segment()";

pub const LOAD_CLASS_FACTS: &str = "ids.compiled_class_facts = segments.add()\nids.n_compiled_class_facts = \
                                    len(os_input.compiled_classes)\nvm_enter_scope({\n    'compiled_class_facts': \
                                    iter(os_input.compiled_classes.items()),\n})";

pub const LOAD_DEPRECATED_CLASS_FACTS: &str =
    "# Creates a set of deprecated class hashes to distinguish calls to deprecated entry \
     points.\n__deprecated_class_hashes=set(os_input.deprecated_compiled_classes.keys())\nids.compiled_class_facts = \
     segments.add()\nids.n_compiled_class_facts = len(os_input.deprecated_compiled_classes)\nvm_enter_scope({\n    \
     'compiled_class_facts': iter(os_input.deprecated_compiled_classes.items()),\n})";

pub const LOAD_DEPRECATED_CLASS_INNER: &str =
    "from starkware.starknet.core.os.contract_class.deprecated_class_hash import (\n    \
     get_deprecated_contract_class_struct,\n)\n\ncompiled_class_hash, compiled_class = \
     next(compiled_class_facts)\n\ncairo_contract = get_deprecated_contract_class_struct(\n    \
     identifiers=ids._context.identifiers, contract_class=compiled_class)\nids.compiled_class = \
     segments.gen_arg(cairo_contract)";

pub const CHECK_DEPRECATED_CLASS_HASH: &str =
    "from starkware.python.utils import from_bytes\n\ncomputed_hash = ids.compiled_class_fact.hash\nexpected_hash = \
     compiled_class_hash\nassert computed_hash == expected_hash, (\n    \"Computed compiled_class_hash is \
     inconsistent with the hash in the os_input. \"\n    f\"Computed hash = {computed_hash}, Expected hash = \
     {expected_hash}.\")\n\nvm_load_program(compiled_class.program, ids.compiled_class.bytecode_ptr)";

/// This is the equivalent of nondet %{ os_input.general_config.sequencer_address %}
pub const SEQUENCER_ADDRESS: &str = "memory[ap] = to_felt_or_relocatable(os_input.general_config.sequencer_address)";
