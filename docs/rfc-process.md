# Introduction
Features that either change the VRR protocol or require significant changes to the codebase should
have an RFC. The rules for this are loose and subject to interpretation. Simple refactorings, even
if they touch a large number of modules may not require an RFC. Use your best judgement. During code
review, a reviewer might look at the change and decide that it is so complex that it requires an
RFC, but hopefully that can be determined before any code is written. If in doubt, open an issue for
the proposed change and ask if it requires an RFC.

The RFC format itself is roughly based off of the [Rust language RFC
template](https://github.com/rust-lang/rfcs/blob/master/0000-template.md). Note that the *How We
Teach This* section is not needed. Also note that the only headers required are *Start Date* and
*Issue number*. RFCs in progress will live in the `rfcs` subdirectory of the haret root. Once they
are either accepted or rejected they should be moved to the corresponding subfolder under `rfcs`.

# RFC Lifecycle

1. New RFCs should be introduced by forking the repository and creating a markdown file named
   `N-descriptive-file-name.md` in the `rfcs` directory where N is the number of the new RFC. Note
   that N should just be an increment of the last RFC open for review. It's unlikely that we will
   have many RFCs in flight at the same time, but if for some reason 2 are open with the same number
   we will request that the author who submitted the later PR pushes a commit changing it.

2. The RFC should be written and a pull request should be opened.

3. A review process will be underway for a short while depending upon workload of reviewers, urgency
   of the request, and complexity of the RFC. Reviewers will comment, and the author is expected to
   update the RFC doc and push new commits as needed. **Please do not rebase or alter commit
   history** during the RFC process.

4. Eventually consensus will be achieved either accepting or rejecting the RFC. If no consensus is
   reached, the core committers to haret will make a final decision.

5. The final RFC should be moved to the accepted or rejected folder and a new commit pushed.

6. The pull request containing the RFC in the accepted or rejected folder will be merged. The
   decision to merge all RFCs, including rejected ones, was done in order to simplify the search for
   knowledge among developers about important parts of the history of the project without requiring
   them to dig through commit logs.
