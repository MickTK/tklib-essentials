module TKLib
	# Http request
	class Request
		# Check if the device is connected to internet
		@@request_connection_status = DLL_func.new("request_connection_status")
		def self.connection_status()
			return @@request_connection_status.call()
		end

		# Perform a get request
		@@request_get = DLL_func.new("request_get")
		def self.get(url, params={})
			validate url => String
			validate params => Hash
			par = []
			params.each do |key, value|
				par.push(key.to_s)
				par.push(value.to_s)
			end
			return @@request_get.call([url,par])
		end

		# Perform a post request
		@@request_post = DLL_func.new("request_post")
		def self.post(url, params={}, form={})
			validate url => String
			validate params => Hash
			validate form => Hash
			par = []
			params.each do |key, value|
				par.push(key.to_s)
				par.push(value.to_s)
			end
			f = []
			form.each do |key, value|
				f.push(key.to_s)
				f.push(value.to_s)
			end
			return @@request_post.call([url,par,f])
		end
	end
end
