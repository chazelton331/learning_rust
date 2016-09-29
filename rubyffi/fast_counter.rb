require 'ffi'

module FastCounter

  extend FFI::Library

  ffi_lib 'target/release/libembed.dylib'

  attach_function :process, [], :void

end

FastCounter.process
