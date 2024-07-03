module TKLib

	DLL_PATH = "Plugins/TKLib/bin/tklib.dll"

	# Win32API wrapper
	class DLL_func
		attr_reader :binding

		def initialize(function, arg="p", ret="p")
			validate function => String
			validate arg => String
			validate ret => String
			@binding = Win32API.new(DLL_PATH, function, arg, ret)
		end

		# Return the value from the function or raise an exception
		# Exception response will be like: {"error": "Error message."}
		def call(args=nil)
			ret = Json.decode(@binding.call(Json.encode(args)))
			raise TKLibError.new(ret[:error].to_s) if ret.is_a?(Hash) && ret.length() == 1 && ret.key?(:error)
			return ret
		end
	end

	class TKLibError < StandardError
		def initialize(msg="TKLib call error.")
			super(msg)
		end
	end
end
