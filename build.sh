rm -rf dist
rm -rf src/pkg

cd native

wasm-pack build --target nodejs

cd ..

cp -r native/pkg src

rm src/pkg/package.json

yarn tsup-node

cp hypua_node_native_bg.wasm dist
