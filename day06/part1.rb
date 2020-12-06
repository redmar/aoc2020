require 'set'
puts File.read("input.txt")
		.split("\n\n")
		.map { |line| Set[*line.gsub("\n","").chars].size }
		.sum
