
// main AActor template for cpp file
let actor_part = Output.part("aactor-template-cpp.txt");

actor_part.put("name", Name);
Output.embed(actor_part);
Output.write_to(Name + ".cpp");

Output.clear();

// main AActor template for header file
let actor_part = Output.part("aactor-template-h.txt");
actor_part.put("name", Name);

// multiple inheritance just gets pasted in -- solve it yourself
if any(OtherInherits) {
    actor_part.put("other_inherits", ", " + join(OtherInherits));
} else {
    actor_part.put("other_inherits", "");
}

actor_part.put("class_properties", join(ClassProperties));

// Field declarations

let field_decls = Output.snippet();

for (field, count) in Input {
    let field_part = Output.part("field-declaration-template.txt");

    // field properties are all collected from block above
    field_part.put("properties", join(field.properties));

    // we use a separate name template to add `m_`, for example
    let name_part = Output.part("field-name-template.txt");
    name_part.put("name", field.name);
    field_part.put("name", name_part.finish());

    // we are currently running with a limited type-system
    // distinguishing primitive types and a couple of complex ones
    // that we template by hand (every complex type has its own template)
    let type_part = Output.part("type-template-" + field.type.pure() + ".txt");
    if field.type.is_primitive() {
        type_part.put("type", field.type.to_string());
    } else {
        type_part.put("type", field.type.inner().pure());
    }
    field_part.put("type", type_part.finish());
    
    // value is optional
    if !field.value.is_empty() {
        field_part.put("value", " = " + field.value.as_value());
    } else {
        field_part.put("value", "");
    }

    field_decls.add(field_part.finish());
}

actor_part.put("field_declarations", field_decls);

// Field definitions

let field_defs = Output.snippet();

for (field, count) in Input {
    let field_part = Output.part("field-definition-template.txt");
    field_part.put("name", field.name);

    let name_part = Output.part("field-name-template.txt");
    name_part.put("name", field.name);
    field_part.put("var", name_part.finish());

    let type_part = Output.part("type-template-" + field.type.pure() + ".txt");
    if field.type.is_primitive() {
        type_part.put("type", field.type.to_string());
    } else {
        type_part.put("type", field.type.inner().pure());
    }
    field_part.put("type", type_part.finish());
    
    field_defs.add(field_part.finish());
}

actor_part.put("field_definitions_and_accessors", field_defs);

Output.embed(actor_part);
Output.write_to(Name + ".h");