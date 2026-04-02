cask "passero" do
  version "0.0.0"

  url "https://github.com/fredsmith/Passero/releases/download/v#{version}/Passero_#{version}_aarch64.dmg"
  sha256 "placeholder" # :arm64

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
