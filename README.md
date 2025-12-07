## Sazycimde <-> datni
<!--TODO LOJBAN: Propery words-->
User interface <-> data

The model of UI used in this project is as written below.
The relationship between each stage is never prescribed to be architectural.
Meaning, code following this doesn't necessarily have to seperate any stage from any other stage
if it doesn't want to.
For example, performance may mean blurring the lines a bit helps significantly which is
absolutely OK.

### Stage pa: External (data source)
- Stage pa (external): Information Data poi external invariants zi'e poi unknown precision .i This is completely external to user
<!-- TODO LOJBAN -->
- Stage 1 (external): Information Data with invariants and unknown precision

This is the ingestion of data.
Think a HTTP request, reading from a file, or any other way importing raw information in any format.
This can include unstructured data, corrupted data, or data without any inherent invariants.
You should allow anything to enter here, in any manner.

### Stage re: External -> User Internal be .ah. be'o (adding invariants, checking, testing)
User Internal is a tanru

- Stage re (internal user): Structured Data poi internal user invariants zi'e poi absolute precision .i
<!-- TODO LOJBAN -->
- Stage 2 (internal user): Structured data that enforces user's invariants, maintaining absolute precision.

Only internal transformations, to uphold `ah` internal invariants.
This may be programmed by the user of course, but must meet `ah` standards.

This mostly enforces invariants.
This must be easily debugged.
This step can be called "parsing", "validation", "sanitation", "normalization", or any similar meaning.
It adds structure, invariants, descriptions, and precision.

For example, the date format as input may be simply a Unix timestamp, but after this step
to maintain perfect precision the proper `ah` datetime format is required which is
perfectly precise about everything including timezone information, which much now be explicitely
set to zo'e or zu'i.

After this step, all `ah` invariants must be upheld.

### Stage ci: User Internal -> Internal User (filtering and presentably translating)
Internal User is a tanru.

Now, the structured data is filtered and transformed to glean useful information
*specific to the user* use case.
