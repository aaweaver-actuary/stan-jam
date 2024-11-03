## Parsing Include Directives

### Goal
Extend the `StanSourceParser` to identify `#include` directives in Stan models, locate files in specified folders, and inline the content of these files into the main model.

#### Identify Include Directives:

- Write a test for a new method (e.g., `parse_includes()`) to confirm that it can detect and capture all `#include` directives from the Stan model text.
   - This method should return a list of included file paths or an error if the syntax is incorrect.

- **Test cases:**
    - The test should include a model text with multiple `#include` directives.
    - The test should include a model text with no `#include` directives.
    - The test should include a model text with an incorrect `#include` directive.
    - The test should include a model text with an `#include` directive that is not a string.
    - The test should include a model text with an `#include` directive that is not a valid file path.

#### Locate included files:
- Write a test that validates the method to search folders (in `StanSourceParser.folders`) to check if it finds included files.

- **Test cases:**
  - file exists in the first folder
  - file only exists in a subfolder
  - file does not exist in any folders

## Inlining included code

### Goal
Inline the content from included files by inserting their content directly into the main model's text.

#### Read File Content:
- Write a test for a method (e.g., `StanSourceParser.read_includes()`) that takes a list of file paths (from `parse_includes()`) and reads the content of each.
- Ensure it correctly handles errors, such as missing files or read permissions.

#### Replace Include Directives with Content:
- Write a test for the replacement function to validate it can replace each `#include` directive in the main Stan model text with the corresponding file’s content.
- Confirm it handles various edge cases, such as:
  - empty files 
  - nested includes
  - malformed paths


## Building the Final Model Text

### Goal
Assemble the Stan model text in the correct order, ensuring all includes are resolved and inlined.

#### Assemble Model Text:
- Write a test for a method like `build_final_model()`, which uses parsed blocks from the StanModel struct and inlines any include content.
- This test should confirm that `build_final_model()` correctly concatenates all model sections with resolved includes in order.
        
#### Validate Final Model:
- Write tests for valid and invalid Stan syntax (perhaps using regex for basic validation).
  - If Stan provides a CLI validator, you could integrate a basic check (e.g., validate_syntax()) to ensure correctness.


## Testing Edge Cases and Refactoring

### Goal
Refactor for efficiency, handle edge cases, and improve modularity following SOLID principles.

#### Handle Recurring Includes:
- Test that the program correctly handles duplicate includes without reloading or re-parsing files unnecessarily.

#### Optimize File Loading:
- Test that files already read once aren’t re-read.

#### Parallelize File Reading (Optional, Performance Optimization):
- If your Stan models get large, consider parallelizing the file reading. 
- Write tests to ensure this doesn’t break ordering.


