import SharedTypes
import SwiftUI

struct ContentView: View {
  @ObservedObject var core: Core
  @StateObject private var audioInputManager = AudioInputManager()

  var body: some View {
    VStack {
      VStack {
        Text(core.view.pitch)
              .font(.title)
              .bold(true)
        Text(core.view.diff)
      }
    }

    .task {
      do {
        if let audioStream = try await audioInputManager.startMonitoring() {
          for await audioData in audioStream {
            core.update(.detectPitch(audioData))
          }
        }
      } catch {
        print("Error starting audio monitoring: \(error)")
      }
    }
  }
}

struct ActionButton: View {
  var label: String
  var color: Color
  var action: () -> Void

  init(label: String, color: Color, action: @escaping () -> Void) {
    self.label = label
    self.color = color
    self.action = action
  }

  var body: some View {
    Button(action: action) {
      Text(label)
        .fontWeight(.bold)
        .font(.body)
        .padding(EdgeInsets(top: 10, leading: 15, bottom: 10, trailing: 15))
        .background(color)
        .cornerRadius(10)
        .foregroundColor(.white)
        .padding()
    }
  }
}

struct ContentView_Previews: PreviewProvider {
  static var previews: some View {
    ContentView(core: Core())
  }
}
