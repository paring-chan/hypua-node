import ffi from 'ffi-napi'
import path from 'path'

const hypua = ffi.Library(
  path.resolve(
    __dirname,
    '..',
    'native',
    'target',
    'release',
    'libhypua_node_native'
  ),
  {
    to_ipf_string: ['string', ['string']],
  }
)

export const toIpfString = (str: string) => hypua.to_ipf_string(str)
