import path from 'path'
import native from './pkg/hypua_node_native'

const wasmPath = path.resolve(
  __dirname,
  '..',
  'native_libs',
  'hypua_node_native.wasm'
)

export const toIpfString = (str: string) => {
  return native.to_ipf_string(str)
}
