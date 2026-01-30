# This formula will be auto-updated by the release process
# To install locally: brew install --build-from-source homebrew/cenv.rb

class Cenv < Formula
  desc "C.env - A simple configuration language and interpreter"
  homepage "https://github.com/rzorzal/.c.env"
  license "MIT"

  # These will be updated automatically by the release script
  version "0.1.2"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/rzorzal/.c.env/releases/download/v#{version}/cenv-Darwin-aarch64.tar.gz"
      sha256 "UPDATE_THIS_SHA256_AFTER_RELEASE"
    else
      url "https://github.com/rzorzal/.c.env/releases/download/v#{version}/cenv-Darwin-x86_64.tar.gz"
      sha256 "UPDATE_THIS_SHA256_AFTER_RELEASE"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/rzorzal/.c.env/releases/download/v#{version}/cenv-Linux-aarch64.tar.gz"
      sha256 "UPDATE_THIS_SHA256_AFTER_RELEASE"
    else
      url "https://github.com/rzorzal/.c.env/releases/download/v#{version}/cenv-Linux-x86_64.tar.gz"
      sha256 "UPDATE_THIS_SHA256_AFTER_RELEASE"
    end
  end

  def install
    bin.install "cenv"
  end

  test do
    # Create a simple test file
    (testpath/"test.cenv").write <<~EOS
      message = "Hello from Homebrew"
      print(message)
    EOS

    assert_match "Hello from Homebrew", shell_output("#{bin}/cenv #{testpath}/test.cenv")
  end
end
