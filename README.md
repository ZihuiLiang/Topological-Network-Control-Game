This is a project for verify the patterns of 2-path cases in Topological Network Control games with neighbourhood parameter $t=1$. We provide a crate named solve_linearforest consisting of:
+  a memorized searching algorithm named Searcher (computing the outcome of OPT(C) where C is a given configuration)
+  a pattern solver named two_path_pattern_solver (determing if C is a draw configuration by patterns where C is a given 2-path configuration.the pattern is that  $\\{Path_{\ell_1},Path_{\ell_2}\\}$ $\text{ }_{\ell_2\ge \ell_1\ge 1}$ is a draw configuration iff $\\{\ell_1, \ell_2\\}\in \\{\\{2,2\\},\\{2,6x\\},\\{1,1\\},\\{7,7\\},\\{7,6x+5\\},\\{6x+5,6y+5\\},\\{6x,6y\\}\\}_{x,y\ge1}$)

In `src/main`, we provide a program to verify the patterns with the results (of configuration $\\{Path_{\ell_1}, Path_{\ell_2}\\}_{1\le \ell_1\le \ell_2< \mathbf{MAX\_SIZE}=100}$) searched by the searching algorithm.

Please make sure that the rust environment (`cargo` command) and Chrome browser are installed.

To install rust environment, please see https://www.rust-lang.org/tools/install.

To run the project, run the following terminal command within serveral seconds :

```bash
cargo run --release
```

To get the document (detailed explanation of the code) of the project, run the following terminal command:

```bash
cargo doc --open --all
```

Then a webpage showing the document will be opened. Please make sure to access this webpage with Chrome browser.
