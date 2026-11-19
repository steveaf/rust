
Create functions with cargo:

cd to where you want the parent folder from cargo!

cd ./exercises

```
mkdir -p functions
if [ -d "functions/src" ]; then
  echo "you already did that, Steve"
else
  cargo new functions
  cd functions
  echo "go code main.rs now!"
  code src/main.rs
fi
```
