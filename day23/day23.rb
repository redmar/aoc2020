class CrabGame
  attr_accessor :ring, :cup_count, :current_cup, :max_cup

  def initialize(elems)
    ((elems.max()+1)..1_000_000).each do |idx|
      elems.push(idx)
    end
    ring = elems.zip(elems.rotate(1)).to_h
    @cup_count = ring.count
    @ring = ring
    @current_cup = elems.first
    @max_cup = elems.max
  end

  def make_move()
    removed_cups = remove_next_three_cups(self.current_cup)
    destination = self.get_destination_cup(removed_cups)
    move_three_cups_to(destination, removed_cups)
    self.current_cup = self.ring[current_cup]
  end

  def remove_next_three_cups(current_cup)
    next_of_current_cup = self.ring[current_cup]
    e1 = self.ring[next_of_current_cup]
    e2 = self.ring[e1]
    e3 = self.ring[e2]
    self.ring.delete(next_of_current_cup)
    self.ring.delete(e1)
    self.ring.delete(e2)
    self.ring[current_cup] = e3
    ret = [next_of_current_cup, e1, e2]
    ret
  end

  def move_three_cups_to(destination_cup, three_cups)
    next_of_dest_cup = self.ring[destination_cup]
    self.ring[destination_cup] = three_cups[0]
    self.ring[three_cups[0]] = three_cups[1]
    self.ring[three_cups[1]] = three_cups[2]
    self.ring[three_cups[2]] = next_of_dest_cup
  end

  def wrapped_decrement(destination_cup)
    destination_cup = destination_cup - 1
    if destination_cup == 0
      self.max_cup
    else
      destination_cup
    end
  end

  def get_destination_cup(three_cups)
    destination_cup = wrapped_decrement(self.current_cup)
    while three_cups.member?(destination_cup) do
      destination_cup = wrapped_decrement(destination_cup)
    end
    destination_cup
  end
end

elems = [8, 7, 1, 3, 6, 9, 4, 5, 2]
game = CrabGame.new(elems)

puts "moving..."

10000000.times do
  game.make_move()
end

el1 = game.ring[1]
el2 = game.ring[el1]
puts "Part2     = #{el1 * el2}"
