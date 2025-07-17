import AVFoundation

class AudioInputManager: ObservableObject {
  private var audioEngine = AVAudioEngine()

  func startMonitoring() async throws -> AsyncStream<[Double]>? {
    let granted = await AVAudioApplication.requestRecordPermission()
    guard granted else {
      return nil
    }

    let format = audioEngine.inputNode.outputFormat(forBus: 0)
    let audioStream = AsyncStream<[Double]> { continuation in
      audioEngine.inputNode.installTap(onBus: 0, bufferSize: 1024, format: format) {
        [] (buffer, audioTime) in
        guard let channelData = buffer.floatChannelData else { return }
        let channel0 = channelData[0]
        let frameLength = Int(buffer.frameLength)
        var newSamples: [Double] = []
        for i in 0..<frameLength {
          // FIXME: once rust pitch crate uses generics, we can just send the f32 without conversion.
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
