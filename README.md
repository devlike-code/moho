# Moho

Moho (spanish for "rust") is a rust-based code generator for creating Unreal Engine-specific C++ files.

## How it works

To use `moho`, create one or more `.moho` files and then run:

```
moho -r <folder where your moho files are>
```

In this repo, you can run `moho -r ./test` to see the generated files.

Calling `moho` will generate `.h` and `.cpp` files within that folder next to the `.moho` files. Here's a sample for a `.moho` file:

```
class ATestActor : AActor
{
    [EditAnywhere, BlueprintReadWrite]
    {
        [Category="Object References"]
        {
            [Description="A nice long description goes here"]
            AAnotherActor* AnotherActor;

            [Description="Ugly af"]
            AYetAnotherActor* YetAnother;
        }

        [Category="Settings"]
        {
            [ClampMin = 10, ClampMax = 15]
            float MaxHealth = 10;
        }
    }

    [BlueprintReadOnly, Transient]
    {
        [Description="Read only var"]
        bool IsTrue = false;
    }
}
```

When you run `moho` on this, we get the following code created:

```cpp
// ATestActor.h

#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"

UCLASS()
class ATestActor : public AActor
{
	GENERATED_BODY()
	
public:	
	// Sets default values for this actor's properties
	ATestActor();

protected:
	// Called when the game starts or when spawned
	virtual void BeginPlay() override;
private:	
	// Called every frame
	virtual void Tick(float DeltaTime) override;

private:
    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category="Object References", Description="A nice long description goes here")
    TObjectPtr<AAnotherActor> m_AnotherActor;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category="Object References", Description="Ugly af")
    TObjectPtr<AYetAnotherActor> m_YetAnother;

    UPROPERTY(EditAnywhere, BlueprintReadWrite, Category="Settings", ClampMin=10, ClampMax=15)
    float m_MaxHealth = 10;

    UPROPERTY(BlueprintReadOnly, Transient, Description="Read only var")
    bool m_IsTrue = false;

public:
	TObjectPtr<AAnotherActor>& GetAnotherActor() const { return m_AnotherActor; }
	const TObjectPtr<AAnotherActor>& GetAnotherActor() const { return m_AnotherActor; }
	void SetAnotherActor(TObjectPtr<AAnotherActor>& value) { m_AnotherActor = value; }

	TObjectPtr<AYetAnotherActor>& GetYetAnother() const { return m_YetAnother; }
	const TObjectPtr<AYetAnotherActor>& GetYetAnother() const { return m_YetAnother; }
	void SetYetAnother(TObjectPtr<AYetAnotherActor>& value) { m_YetAnother = value; }

	float& GetMaxHealth() const { return m_MaxHealth; }
	const float& GetMaxHealth() const { return m_MaxHealth; }
	void SetMaxHealth(float& value) { m_MaxHealth = value; }

	bool& GetIsTrue() const { return m_IsTrue; }
	const bool& GetIsTrue() const { return m_IsTrue; }
	void SetIsTrue(bool& value) { m_IsTrue = value; }
};
```

```cpp
// ATestActor.cpp

#include "ATestActor.h"

// Sets default values
ATestActor::ATestActor()
{
    // Set this actor to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
    PrimaryActorTick.bCanEverTick = true;
}

// Called when the game starts or when spawned
void ATestActor::BeginPlay()
{
    Super::BeginPlay();
}

// Called every frame
void ATestActor::Tick(float DeltaTime)
{
    Super::Tick(DeltaTime);
}
```

You can also use `moho` to create a new `.moho` file automatically by using the `-g` flag. Use `-n` to skip being prompted for a file name (optional).

```
moho -r <target folder> -g <superclass> [-n <your class name>]
```

The `-g` version of this command won't run the code generator after creating the file.

## Yes, but how does it _work_?

`moho` is actually three separate things in a trench-coat. Let's go into details. It goes like this:

```
    Parser --> Scripting Engine --> Templating
```

### Parser

First, there's a **parser** that allows us to one or more classes in a translation unit. A class contains a number of blocks and fields, and all of this can have **properties** on it. Here's an example of a block with properties:

```
    [EditAnywhere, BlueprintReadWrite]
    {
        ...
    }
```

If a block contains a field, the field inherits all of the block's properties. If a class has properties, these are *not* inherited by the fields inside.

If your properties have values, use the `A=B` notation. The values don't have to be strings - you can have integers, chars, bools, floats, string literals, and a useless nullptr. Expressions are *not* allowed.

### Scripting Engine

Once the parser does what it does best, the produced result is a collection of classes, blocks, and fields. It's now time to create a [Rhai](https://rhai.rs/) engine per class found, and run a script on it. The script that should run depends on the superclass your class is inheriting. For example, if you chose to extend `AActor`, the engine will look for `AActor.rhai` in the configuration folder of `moho`, and run it. If there's no superclass, `base.rhai` will be called (it does **nothing**).

Some values are preset and used to generate the code we want to end up with:

- **Filename**: the name of the file you're reading;
- **Path**: the directory the file is in (running `moho` will always do a recursive check for all `moho` files);
- **Name**: the name of the actual class;
- **Input**: the normalized fields in this class (as if all blocks with properties have been flattened);
- **ClassProperties**: the properties the class has on it;
- **Inherit**: the superclass we're extending;
- **OtherInherits**: if multiple inheritance is being used, the other superclasses are a part of this array;

A number of functions and classes are also available. To actually generate code, you need to use the `Output` variable in your script. Here's an example:

```rust
// This adds a line of strings into the output. Output is accumulated!
Output.add("// hello!");

// We create a template part - this is a text file that has a {{name}} field somewhere in it.
let actor_part = Output.part("aactor-template-cpp.txt");

// We can fill the fields with values
actor_part.put("name", Name);

// We can then embed the filled template into the output
Output.embed(actor_part);

// This writes the contents of the output into the given file
Output.write_to(Name + ".cpp");

// At the end of each part, we can clear the output and start anew!
Output.clear();
```

For a full example of how to use the scripting and templating, look at the `AActor.rhai` script. It creates a full source/header file for the `AActor` superclass, and shows off some parts like nested templates, array joins, and the complexities of types, blocks, and template parts.

### Templating

Here's what a template looks like:

```cpp
#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Actor.h"

UCLASS({{class_properties}})
class {{name}} : public AActor{{other_inherits}}
{
	GENERATED_BODY()
	
public:	
	// Sets default values for this actor's properties
	{{name}}();

protected:
	// Called when the game starts or when spawned
	virtual void BeginPlay() override;
private:	
	// Called every frame
	virtual void Tick(float DeltaTime) override;

private:
{{field_declarations}}

public:
{{field_definitions_and_accessors}}
};
```

The language is quite simple: all the template parts are made in `{{these}}`. To nest templates, there are three tools:

- **OutputWriter** (an instance of this is made for every script run, in the `Output` variable) has an `add(String)` method that concats a new piece to the output log. When done, we can `Output.write_to(filename)` to generate code. As mentioned before, you can `Output.clear()` the output to start from scratch.

- **OutputTemplate** can be created from `Output.part(filename)`. You can use `Output.embed(template)` to embed the result of a filled template back into the output. If you don't want to embed _directly_ into `Output`, you can use `template.finish()` to simply get the string back and use it how ever you wish.

- **StringWriter** can be created from `Output.snippet()`. This is very similar to the output writer, but without the complexity behind the scenes (the `OutputWriter` has to know about configurations and folders). `StringWriter` also has `add` to add strings to it, and `get` to get the string back.

Here's a simple example of all of these:

```rust
// We create a full-file template for the AActor class
let actor_part = Output.part("aactor-template-h.txt");

// We need to fill in multiple fields, so we create a snippet to collect them
let field_decls = Output.snippet();

// Loop through the fields from our class input fields
for (field, _) in Input {
    // Generate a field declaration template per field
    let field_part = Output.part("field-declaration-template.txt");

    // Field properties are all collected from all the blocks above it 
    field_part.put("properties", join(field.properties));

    // We might want to apply a separate name template (to add `m_`, for example)
    let name_part = Output.part("field-name-template.txt");
    name_part.put("name", field.name);
    field_part.put("name", name_part.finish());

    // We add this field part into our field declaration snippet
    field_decls.add(field_part.finish());
}

// Once all of our fields are done, we put them into our larger template
actor_part.put("field_declarations", field_decls);

// We embed the larger template into the output
Output.embed(actor_part);

// We write the output into a file
Output.write_to(Name + ".h");
```

## Something undefined?

First run `moho --help` to check whether that may help you. Then ping me or open a new issue!
