n, p, s = gets.split().map(&:to_i)
for i in 1..s
  if gets.split().map(&:to_i)[1..].include?(p)
    puts 'KEEP'
  else
    puts 'REMOVE'
  end
end
