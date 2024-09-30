if(UNIX)
  # Can't compile Cinder under linux because linux is macro set to 1
  add_definitions(-Ulinux)
endif(UNIX)

CPMAddPackage(
  NAME Cinder
  GITHUB_REPOSITORY cinder/Cinder
  GIT_TAG master
)
