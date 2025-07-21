import AVFoundation

class AudioInputManager: ObservableObject {
  private var audioEngine = AVAudioEngine()
  private let bufferSize: UInt32
  private let format: AVAudioFormat
  public let sampleRate: Double

  init(bufferSize: UInt32 = 1024) {
    self.bufferSize = bufferSize
    self.format = audioEngine.inputNode.outputFormat(forBus: 0)
    self.sampleRate = format.sampleRate
  }

  func startMonitoring() async throws -> AsyncStream<[Double]>? {
    let granted = await AVAudioApplication.requestRecordPermission()
    guard granted else {
      return nil
    }

    let audioStream = AsyncStream<[Double]> { continuation in
      audioEngine.inputNode.installTap(onBus: 0, bufferSize: self.bufferSize, format: self.format) {
        [] (buffer, audioTime) in
        guard let channelData = buffer.floatChannelData else { return }
        let channel0 = channelData[0]
        let frameLength = Int(buffer.frameLength)
        var newSamples: [Double] = []
        for i in 0..<frameLength {
          // FIXME: once rust pitch crate uses generics, we can just send f32 values without conversion.
          // Perhaps monitor before and after if this makes any difference on the CPU.
          newSamples.append(Double(channel0[i]))
        }
        continuation.yield(newSamples)
      }
    }
    try audioEngine.start()
    print("Audio engine started.")
    return audioStream
  }

  func stopAudioInput() {
    if audioEngine.isRunning {
      audioEngine.stop()
      audioEngine.inputNode.removeTap(onBus: 0)  // Remove the tap when stopping
      print("Audio engine stopped.")
    }
  }
}
