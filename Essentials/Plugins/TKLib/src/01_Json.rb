module TKLib
  
	# Json encoder/decoder
	class Json
		@@json_encode = DLL_func.new("json_encode")
		def self.encode(data)
			ret = @@json_encode.binding.call({:payload => data}.to_s)[11..-2]
			raise "Encoding error." if ret == nil && data != nil
			return ret
		end

		@@json_decode = DLL_func.new("json_decode")
		def self.decode(data)
			validate data => String
			ret = eval(@@json_decode.binding.call(data))
			raise "Decoding error." if ret == nil && data != "null"
			return ret
		end
	end
end
