const PLATFORM_PACKAGES: Record<string, string> = {
  'linux-x64': '@ancora/sdk-linux-x64-gnu',
  'darwin-x64': '@ancora/sdk-darwin-x64',
  'darwin-arm64': '@ancora/sdk-darwin-arm64',
  'win32-x64': '@ancora/sdk-win32-x64-msvc',
  'linux-arm64': '@ancora/sdk-linux-arm64-gnu',
}

export function loadBinding(): unknown {
  const platformKey = `${process.platform}-${process.arch}`
  const platformPkg = PLATFORM_PACKAGES[platformKey]

  if (platformPkg) {
    try {
      // eslint-disable-next-line @typescript-eslint/no-var-requires
      return require(platformPkg) as unknown
    } catch {
    }
  }

  try {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    return require('./ancora.node') as unknown
  } catch {
    throw new Error(
      `ancora native module not found for ${platformKey}. ` +
      'Run "npm run build" or install the platform-specific package.'
    )
  }
}
