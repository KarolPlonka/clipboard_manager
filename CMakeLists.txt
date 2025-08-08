cmake_minimum_required(VERSION 3.10)
project(RustAppWithGPaste)

# Set the installation prefix if not already set
if(CMAKE_INSTALL_PREFIX_INITIALIZED_TO_DEFAULT)
    set(CMAKE_INSTALL_PREFIX "/usr/local" CACHE PATH "Installation prefix" FORCE)
endif()

# Find required packages
find_package(PkgConfig REQUIRED)

# Check for GNOME environment
execute_process(
    COMMAND bash -c "echo $XDG_CURRENT_DESKTOP"
    OUTPUT_VARIABLE CURRENT_DESKTOP
    OUTPUT_STRIP_TRAILING_WHITESPACE
)

execute_process(
    COMMAND bash -c "echo $DESKTOP_SESSION"
    OUTPUT_VARIABLE DESKTOP_SESSION
    OUTPUT_STRIP_TRAILING_WHITESPACE
)

# Check if gsettings is available
find_program(GSETTINGS gsettings)
if(NOT GSETTINGS)
    message(FATAL_ERROR "gsettings not found. This application requires GNOME.")
endif()

# Verify GNOME is running
if(NOT CURRENT_DESKTOP MATCHES "GNOME" AND NOT DESKTOP_SESSION MATCHES "gnome")
    message(WARNING "GNOME desktop environment not detected. This application is designed for GNOME.")
    message(WARNING "Current desktop: ${CURRENT_DESKTOP}")
    message(WARNING "Desktop session: ${DESKTOP_SESSION}")
    
    # Check if GNOME Shell is at least installed
    pkg_check_modules(GNOME_SHELL gnome-shell)
    if(NOT GNOME_SHELL_FOUND)
        message(FATAL_ERROR "GNOME Shell is not installed. This application requires GNOME.")
    endif()
endif()

# Check if cargo is available
find_program(CARGO cargo)
if(NOT CARGO)
    message(FATAL_ERROR "Cargo not found. Please install Rust and Cargo.")
endif()

# Check if gpaste-2 is already installed
# Try different package names that might be used
pkg_check_modules(GPASTE QUIET gpaste-2)
if(NOT GPASTE_FOUND)
    pkg_check_modules(GPASTE QUIET gpaste)
endif()
if(NOT GPASTE_FOUND)
    pkg_check_modules(GPASTE QUIET libgpaste)
endif()

# Alternative: Check if gpaste-client is available as a fallback
if(NOT GPASTE_FOUND)
    find_program(GPASTE_CLIENT gpaste-client)
    if(GPASTE_CLIENT)
        message(STATUS "Found gpaste-client at: ${GPASTE_CLIENT}")
        set(GPASTE_FOUND TRUE)
    endif()
endif()

# Check for GPaste GIR files or libraries directly
if(NOT GPASTE_FOUND)
    find_path(GPASTE_GIR_DIR
        NAMES GPaste-2.gir
        PATHS
            /usr/share/gir-1.0
            /usr/local/share/gir-1.0
            /usr/share/gnome/gir-1.0
    )
    find_library(GPASTE_LIB
        NAMES gpaste gpaste-2
        PATHS
            /usr/lib
            /usr/lib64
            /usr/local/lib
            /usr/local/lib64
            /usr/lib/x86_64-linux-gnu
            /usr/lib/aarch64-linux-gnu
    )
    if(GPASTE_GIR_DIR OR GPASTE_LIB)
        message(STATUS "Found GPaste files: GIR=${GPASTE_GIR_DIR}, LIB=${GPASTE_LIB}")
        set(GPASTE_FOUND TRUE)
    endif()
endif()

# Install gpaste-2 if not found
if(NOT GPASTE_FOUND)
    message(STATUS "GPaste-2 not found. Will attempt to install it.")
    
    # Check for package manager
    find_program(APT apt-get)
    find_program(DNF dnf)
    find_program(YUM yum)
    find_program(PACMAN pacman)
    find_program(ZYPPER zypper)
    
    if(APT)
        message(STATUS "Using apt-get to install gpaste")
        execute_process(
            COMMAND sudo apt-get update
            RESULT_VARIABLE APT_UPDATE_RESULT
        )
        execute_process(
            COMMAND sudo apt-get install -y libgpaste-2-0 libgpaste-dev gpaste gnome-shell-extensions-gpaste
            RESULT_VARIABLE APT_INSTALL_RESULT
        )
        if(NOT APT_INSTALL_RESULT EQUAL 0)
            message(WARNING "Failed to install gpaste via apt-get. You may need to install it manually.")
        endif()
    elseif(DNF)
        message(STATUS "Using dnf to install gpaste")
        execute_process(
            COMMAND sudo dnf install -y gpaste gpaste-devel gnome-shell-extension-gpaste
            RESULT_VARIABLE DNF_RESULT
        )
        if(NOT DNF_RESULT EQUAL 0)
            message(WARNING "Failed to install gpaste via dnf. You may need to install it manually.")
        endif()
    elseif(YUM)
        message(STATUS "Using yum to install gpaste")
        execute_process(
            COMMAND sudo yum install -y gpaste gpaste-devel gnome-shell-extension-gpaste
            RESULT_VARIABLE YUM_RESULT
        )
        if(NOT YUM_RESULT EQUAL 0)
            message(WARNING "Failed to install gpaste via yum. You may need to install it manually.")
        endif()
    elseif(PACMAN)
        message(STATUS "Using pacman to install gpaste")
        execute_process(
            COMMAND sudo pacman -S --noconfirm gpaste gnome-shell-extension-gpaste
            RESULT_VARIABLE PACMAN_RESULT
        )
        if(NOT PACMAN_RESULT EQUAL 0)
            message(WARNING "Failed to install gpaste via pacman. You may need to install it manually.")
        endif()
    elseif(ZYPPER)
        message(STATUS "Using zypper to install gpaste")
        execute_process(
            COMMAND sudo zypper install -y gpaste gpaste-devel gnome-shell-extension-gpaste
            RESULT_VARIABLE ZYPPER_RESULT
        )
        if(NOT ZYPPER_RESULT EQUAL 0)
            message(WARNING "Failed to install gpaste via zypper. You may need to install it manually.")
        endif()
    else()
        message(WARNING "No supported package manager found. Please install gpaste manually.")
    endif()
    
    # Re-check for gpaste after installation attempt
    pkg_check_modules(GPASTE QUIET gpaste-2)
    if(NOT GPASTE_FOUND)
        pkg_check_modules(GPASTE QUIET gpaste)
    endif()
    if(NOT GPASTE_FOUND)
        find_program(GPASTE_CLIENT gpaste-client)
        if(GPASTE_CLIENT)
            set(GPASTE_FOUND TRUE)
        endif()
    endif()
    
    if(NOT GPASTE_FOUND)
        message(FATAL_ERROR "GPaste installation failed or not detected. Please install it manually.")
    endif()
endif()

# Set Rust project variables
set(RUST_PROJECT_NAME "clipboard_manager" CACHE STRING "Name of the Rust project")
set(RUST_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}" CACHE PATH "Path to Rust source directory")
set(RUST_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/target" CACHE PATH "Rust target directory")
set(RUST_BINARY_NAME "${RUST_PROJECT_NAME}" CACHE STRING "Name of the output binary")

# Custom target to build the Rust project
add_custom_target(rust_build ALL
    COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${RUST_TARGET_DIR} ${CARGO} build --release
    WORKING_DIRECTORY ${RUST_SOURCE_DIR}
    COMMENT "Building Rust project in release mode"
)

# Custom target to clean the Rust build
add_custom_target(rust_clean
    COMMAND ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${RUST_TARGET_DIR} ${CARGO} clean
    WORKING_DIRECTORY ${RUST_SOURCE_DIR}
    COMMENT "Cleaning Rust build"
)

# Installation rules
install(
    PROGRAMS ${RUST_TARGET_DIR}/release/${RUST_BINARY_NAME}
    DESTINATION ${CMAKE_INSTALL_PREFIX}/bin
    COMPONENT runtime
)

# Create keybinding setup script
configure_file(
    ${CMAKE_CURRENT_SOURCE_DIR}/setup_keybinding.sh.in
    ${CMAKE_CURRENT_BINARY_DIR}/setup_keybinding.sh
    @ONLY
)

# Add execute permissions to the generated script
file(CHMOD ${CMAKE_CURRENT_BINARY_DIR}/setup_keybinding.sh PERMISSIONS OWNER_READ OWNER_WRITE OWNER_EXECUTE GROUP_READ GROUP_EXECUTE WORLD_READ WORLD_EXECUTE)

# Install keybinding setup script
install(
    PROGRAMS ${CMAKE_CURRENT_BINARY_DIR}/setup_keybinding.sh
    DESTINATION ${CMAKE_INSTALL_PREFIX}/share/${RUST_PROJECT_NAME}
    COMPONENT keybinding
)

# Add custom target to set up keybinding
add_custom_target(setup_keybinding
    COMMAND ${CMAKE_CURRENT_BINARY_DIR}/setup_keybinding.sh
    DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/setup_keybinding.sh
    COMMENT "Setting up GNOME keybinding for ${RUST_BINARY_NAME}"
)

# Add custom target to remove keybinding
add_custom_target(remove_keybinding
    COMMAND ${CMAKE_CURRENT_BINARY_DIR}/setup_keybinding.sh --remove
    DEPENDS ${CMAKE_CURRENT_BINARY_DIR}/setup_keybinding.sh
    COMMENT "Removing GNOME keybinding for ${RUST_BINARY_NAME}"
)

# Desktop file for GNOME
configure_file(
    ${CMAKE_CURRENT_SOURCE_DIR}/${RUST_PROJECT_NAME}.desktop.in
    ${CMAKE_CURRENT_BINARY_DIR}/${RUST_PROJECT_NAME}.desktop
    @ONLY
)

install(
    FILES ${CMAKE_CURRENT_BINARY_DIR}/${RUST_PROJECT_NAME}.desktop
    DESTINATION ${CMAKE_INSTALL_PREFIX}/share/applications
    COMPONENT desktop
)

# Add uninstall target
if(NOT TARGET uninstall)
    configure_file(
        "${CMAKE_CURRENT_SOURCE_DIR}/cmake_uninstall.cmake.in"
        "${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake"
        IMMEDIATE @ONLY
    )
    
    add_custom_target(uninstall
        COMMAND ${CMAKE_COMMAND} -P ${CMAKE_CURRENT_BINARY_DIR}/cmake_uninstall.cmake
        COMMAND ${CMAKE_CURRENT_BINARY_DIR}/setup_keybinding.sh --remove
    )
endif()

# Post-install message
install(CODE "
    message(STATUS \"\")
    message(STATUS \"====================================\")
    message(STATUS \"Installation complete!\")
    message(STATUS \"\")
    message(STATUS \"To set up the keybinding (Ctrl+Alt+V):\")
    message(STATUS \"  Run: make setup_keybinding\")
    message(STATUS \"\")
    message(STATUS \"Or manually run:\")
    message(STATUS \"  ${CMAKE_INSTALL_PREFIX}/share/${RUST_PROJECT_NAME}/setup_keybinding.sh\")
    message(STATUS \"====================================\")
    message(STATUS \"\")
")

# Print configuration summary
message(STATUS "")
message(STATUS "=== Rust App Build Configuration ===")
message(STATUS "Rust project name: ${RUST_PROJECT_NAME}")
message(STATUS "Source directory: ${RUST_SOURCE_DIR}")
message(STATUS "Target directory: ${RUST_TARGET_DIR}")
message(STATUS "Binary name: ${RUST_BINARY_NAME}")
message(STATUS "Install prefix: ${CMAKE_INSTALL_PREFIX}")
message(STATUS "GPaste-2 found: ${GPASTE_FOUND}")
message(STATUS "GNOME detected: ${CURRENT_DESKTOP}")
message(STATUS "===================================")
message(STATUS "")
