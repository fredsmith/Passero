cask "passero" do
  version "2026.4.11-2"

  url "https://github.com/fredsmith/Passero/releases/download/v#{version}/Passero_#{version}_aarch64.dmg"
  sha256 "9fbb7da05f3c95c6f60a5a1a7146b0bd6ee469a2442d6c728a724a52843471d2" # :arm64

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
