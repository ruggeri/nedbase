require 'set'

`rm -rf ./logs`
`mkdir ./logs`

message_log_names = Set.new
lines = File.readlines("./history")

thread_message_logs = Hash.new do |h, id|
  h[id] = Hash.new { |h, msg| h[msg] = [] }
end

lines.each do |line|
  result = /ThreadId\((.*)\): \[(.*)\] .*/.match line

  id, message = result[1], result[2]
  thread_message_logs[id]["all"] << line
  thread_message_logs[id][message] << line
  message_log_names << message
end

thread_message_logs.each do |thread_id, message_logs|
  # Skip completed threads
  next if message_logs["all"].last =~ /thread_terminated/

  message_logs.each do |message, logs|
    File.open("./logs/thread_#{thread_id}_#{message}", "w") do |f|
      f.puts logs
    end
  end
end


message_log_names.each do |msg_log_name|
  File.open("./logs/last_#{msg_log_name}", "w") do |f|
    thread_message_logs.each do |thread_id, message_logs|
      next if message_logs["all"].last =~ /thread_terminated/
      f.puts message_logs[msg_log_name].last
    end
  end
end
