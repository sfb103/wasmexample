# wasmexample

This is a repository that contains simple WebAssembly coding examples. In order to utilize the examples you'll want to do one-time setup your `wasmdevcontainer`. You can do that with:
 
```
./initUserVol.sh y
./setupUser.sh -c
```

Then you can use the `wasmdevcontainer` from a terminal:
```
./shell.sh
```
or from within VSCode:
```
code .
# Then hit "Reopen in container"
```

This will drop you into the `wasmdevcontainer` where there are a bunch of tools and runtimes installed, and support for various programming languages.

For details on the individual examples, see the README.md within each project directory.

[rust/calculator/README.md](./rust/calculator/README.md)<br>
[rust/crosstalk/README.md](./rust/crosstalk/README.md)<br>
[rust/withdraw/README.md](./rust/withdraw/README.md)<br>
[cxx/withdraw/README.md](./cxx/withdraw/README.md)<br>
[go/withdraw/README.md](./go/withdraw/README.md)<br>

Feel free to add examples or edit where you see fit.
