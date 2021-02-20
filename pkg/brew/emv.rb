class Emv < Formula
  desc "EMV tooling"
  homepage "https://github.com/caiotavares/emv"
  url "https://github.com/caiotavares/emv/releases/download/v0.1.0/emv-0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "ffbd5c5f27f707e7aa62b6b67c3e8f69d78c597577c63a3b93d524e94d0c333f"
  license "MIT"

  def install
    bin.install "emv"
  end
end
