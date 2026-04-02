cask "passero" do
  version "2026.4.2-4"

  url "https://github.com/fredsmith/Passero/releases/download/v#{version}/Passero_#{version}_aarch64.dmg"
  sha256 "de315e365a6dab0820d97065987ac38a453144f0b8ac270f0a2a2cab3a478e4f" # :arm64

  name "Passero"
  desc "Desktop GUI for pass, the standard Unix password manager"
  homepage "https://github.com/fredsmith/Passero"

  app "Passero.app"

  caveats <<~EOS
    #{token} is not signed with an Apple Developer certificate.
    On first launch, macOS Gatekeeper will block it. To allow it:
      System Settings > Privacy & Security > scroll down > click "Open Anyway"
    Or run:
      xattr -d com.apple.quarantine /Applications/Passero.app
  EOS

  zap trash: ["~/Library/Application Support/com.fredsmith.passero"]
end
