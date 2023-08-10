```
/usr/libexec/PlistBuddy -c "Add :com.apple.security.get-task-allow bool true" tmp.entitlements
codesign -s - -f --entitlements tmp.entitlements target/debug/coredump-test
```

# References

- https://stackoverflow.com/questions/55736235/how-do-you-debug-a-linux-core-dump-using-vscode
- https://developer.apple.com/forums/thread/694233?answerId=695943022#695943022
- https://crates.io/crates/coredump
