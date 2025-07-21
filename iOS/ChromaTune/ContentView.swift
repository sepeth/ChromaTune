import SharedTypes
import SwiftUI

struct ContentView: View {
  @ObservedObject var core: Core
  @StateObject private var audioInputManager = AudioInputManager(bufferSize: 1024)

  var body: some View {
    VStack {
      VStack {
        Text(core.view.pitch)
          .font(.title)
          .bold(true)
        Text(String(core.view.diff))
      }
    }

    .task {
      do {
        if let audioStream = try await audioInputManager.startMonitoring() {
          for await audioData in audioStream {
            core.update(.detectPitch(audioData, audioInputManager.sampleRate))
          }
        }
      } catch {
        print("Error starting audio monitoring: \(error)")
      }
    }
  }
}

struct ContentView_Previews: PreviewProvider {
  static var previews: some View {
    ContentView(core: Core())
  }
}
