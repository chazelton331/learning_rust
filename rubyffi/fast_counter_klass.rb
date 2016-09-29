require 'ffi'

module FastCounterFunctionality
  extend FFI::Library

  ffi_lib 'target/release/libembed.dylib'

  attach_function :process, [], :void
end


class FastCounterKlass
  include FastCounterFunctionality

  def do_stuff
    process
  end

end

klass = FastCounterKlass.new

klass.do_stuff
