module SlowCounter

  def self.process
    threads = []

    10.times do |i|
      threads << Thread.new do
        count = 0

        5_000_000.times do
          count += 1
        end

        count
      end
    end

    threads.each do |thread|
      puts "Thread finished, count=#{thread.value}"
    end

  end
end

SlowCounter.process
