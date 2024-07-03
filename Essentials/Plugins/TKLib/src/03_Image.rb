module TKLib
  
	# Image manipulation
	class Image

		# Combine two images in one
		# @param bottom: the path of the image used as a base. For example: "Graphics/Pokemon/Front/Arcanine.png"
		# @param top: the path of the image set above the base image
		# @param pos_x: x position
		# @param pos_y: y position
		# @param force: overwrite existent file
		# @return Boolean
		@@image_combine = DLL_func.new("image_combine")
		def self.combine(bottom, top, result, pos_x=0, pos_y=0, force=false)
			return @@image_combine.call([bottom, top, result, pos_x, pos_y, force])
		end
	end
end
