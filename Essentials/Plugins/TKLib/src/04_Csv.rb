module TKLib
  # Read an parse a file to an array of hashmaps
  class Csv
    @@csv_parse = DLL_func.new("csv_parse")
		def self.parse(text,delimiter=",")
      validate text => String
      validate delimiter => String
			return @@csv_parse.call([text,delimiter])
		end
  end
end
