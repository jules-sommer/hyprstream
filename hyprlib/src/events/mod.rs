use tracing::warn;

#[derive(Debug, PartialEq)]
pub enum Event {
  Workspace(Workspace),
  WorkspaceV2(WorkspaceV2),
  FocusedMon(FocusedMon),
  ActiveWindow(ActiveWindow),
  ActiveWindowV2(ActiveWindowV2),
  Fullscreen(Fullscreen),
  MonitorRemoved(MonitorRemoved),
  MonitorAdded(MonitorAdded),
  MonitorAddedV2(MonitorAddedV2),
  CreateWorkspace(CreateWorkspace),
  CreateWorkspaceV2(CreateWorkspaceV2),
  DestroyWorkspace(DestroyWorkspace),
  DestroyWorkspaceV2(DestroyWorkspaceV2),
  MoveWorkspace(MoveWorkspace),
  MoveWorkspaceV2(MoveWorkspaceV2),
  RenameWorkspace(RenameWorkspace),
  ActiveSpecial(ActiveSpecial),
  ActiveLayout(ActiveLayout),
  OpenWindow(OpenWindow),
  CloseWindow(CloseWindow),
  MoveWindow(MoveWindow),
  MoveWindowV2(MoveWindowV2),
  OpenLayer(OpenLayer),
  CloseLayer(CloseLayer),
  Submap(Submap),
  ChangeFloatingMode(ChangeFloatingMode),
  Urgent(Urgent),
  Minimize(Minimize),
  Screencast(Screencast),
  WindowTitle(WindowTitle),
  IgnoreGroupLock(IgnoreGroupLock),
  LockGroups(LockGroups),
  ConfigReloaded,
  Pin(Pin),
}

impl Event {
  pub fn from(event_type: &str, data: &str) -> Self {
    match event_type {
      "workspace" => Event::Workspace(Workspace::parse(data)),
      "workspacev2" => Event::WorkspaceV2(WorkspaceV2::parse(data)),
      "focusedmon" => Event::FocusedMon(FocusedMon::parse(data)),
      "activewindow" => Event::ActiveWindow(ActiveWindow::parse(data)),
      "activewindowv2" => Event::ActiveWindowV2(ActiveWindowV2::parse(data)),
      "moveworkspace" => Event::MoveWorkspace(MoveWorkspace::parse(data)),
      "moveworkspacev2" => Event::MoveWorkspaceV2(MoveWorkspaceV2::parse(data)),
      "fullscreen" => Event::Fullscreen(Fullscreen::parse(data)),
      "monitorremoved" => Event::MonitorRemoved(MonitorRemoved::parse(data)),
      "monitoradded" => Event::MonitorAdded(MonitorAdded::parse(data)),
      "monitoraddedv2" => Event::MonitorAddedV2(MonitorAddedV2::parse(data)),
      "createworkspace" => Event::CreateWorkspace(CreateWorkspace::parse(data)),
      "createworkspacev2" => Event::CreateWorkspaceV2(CreateWorkspaceV2::parse(data)),
      "destroyworkspace" => Event::DestroyWorkspace(DestroyWorkspace::parse(data)),
      "destroyworkspacev2" => Event::DestroyWorkspaceV2(DestroyWorkspaceV2::parse(data)),
      "renameworkspace" => Event::RenameWorkspace(RenameWorkspace::parse(data)),
      "activespecial" => Event::ActiveSpecial(ActiveSpecial::parse(data)),
      "activelayout" => Event::ActiveLayout(ActiveLayout::parse(data)),
      "openwindow" => Event::OpenWindow(OpenWindow::parse(data)),
      "closewindow" => Event::CloseWindow(CloseWindow::parse(data)),
      "movewindow" => Event::MoveWindow(MoveWindow::parse(data)),
      "movewindowv2" => Event::MoveWindowV2(MoveWindowV2::parse(data)),
      "openlayer" => Event::OpenLayer(OpenLayer::parse(data)),
      "closelayer" => Event::CloseLayer(CloseLayer::parse(data)),
      "submap" => Event::Submap(Submap::parse(data)),
      "changefloatingmode" => Event::ChangeFloatingMode(ChangeFloatingMode::parse(data)),
      "urgent" => Event::Urgent(Urgent::parse(data)),
      "minimize" => Event::Minimize(Minimize::parse(data)),
      "screencast" => Event::Screencast(Screencast::parse(data)),
      "windowtitle" => Event::WindowTitle(WindowTitle::parse(data)),
      "ignoregrouplock" => Event::IgnoreGroupLock(IgnoreGroupLock::parse(data)),
      "lockgroups" => Event::LockGroups(LockGroups::parse(data)),
      "configreloaded" => Event::ConfigReloaded,
      "pin" => Event::Pin(Pin::parse(data)),
      _ => {
        warn!(event_type = event_type, data = data, "Unhandled event type");
        Event::ConfigReloaded
      }
    }
  }
}

trait Parse {
  fn parse(data: &str) -> Self;
}

/// `Workspace` is emitted on workspace change. Is emitted ONLY when a
///  user requests a workspace change, and is not emitted on mouse
///  movements (see activemon)
///
/// # Fields
/// * `workspace_name` - Represents the WORKSPACENAME of the workspace.
///
/// # Format
/// workspace>>WORKSPACENAME
#[derive(Debug, Clone, PartialEq)]
pub struct Workspace {
  pub workspace_name: String,
}

impl Parse for Workspace {
  fn parse(data: &str) -> Self {
    Workspace {
      workspace_name: data.to_string(),
    }
  }
}

/// `WorkspaceV2` is emitted on workspace change. Is emitted ONLY when a
///  user requests a workspace change, and is not emitted on mouse
///  movements (see activemon)
///
/// # Fields
///
/// * `workspace_id` - Represents the WORKSPACEID of the workspace.
/// * `workspace_name` - Represents the WORKSPACENAME of the workspace.
///
/// # Format
/// workspace_v2>>WORKSPACEID,WORKSPACENAME
#[derive(Debug, Clone, PartialEq)]
pub struct WorkspaceV2 {
  pub workspace_id: u32,
  pub workspace_name: String,
}

impl Parse for WorkspaceV2 {
  fn parse(data: &str) -> Self {
    let (workspace_id, workspace_name) = data.split_once(",").unwrap();
    WorkspaceV2 {
      workspace_id: workspace_id.parse().unwrap(),
      workspace_name: workspace_name.to_string(),
    }
  }
}

/// `FocusedMon` is emitted when the active monitor is changed.
///
/// # Fields
///
/// * `monitor_name` - Represents the MONNAME / monitor name of the active monitor.
/// * `workspace_name` - Represents the WORKSPACENAME of the active monitor.
///
/// # Format
/// focusedmon>>MONNAME,WORKSPACENAME
#[derive(Debug, Clone, PartialEq)]
pub struct FocusedMon {
  pub monitor_name: String,
  pub workspace_name: String,
}

impl Parse for FocusedMon {
  fn parse(data: &str) -> Self {
    let (monitor_name, workspace_name) = data.split_once(",").unwrap();
    FocusedMon {
      monitor_name: monitor_name.to_string(),
      workspace_name: workspace_name.to_string(),
    }
  }
}

/// `ActiveWindow` is emitted when the active window is changed.
///
/// # Fields
///
/// * `window_class` - Represents the WINDOWCLASS of the active window.
/// * `window_title` - Represents the WINDOWTITLE of the active window.
///
/// # Format
/// activewindow>>WINDOWCLASS,WINDOWTITLE
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveWindow {
  pub window_class: String,
  pub window_title: String,
}

impl Parse for ActiveWindow {
  fn parse(data: &str) -> Self {
    let (window_class, window_title) = data.split_once(",").unwrap();
    ActiveWindow {
      window_class: window_class.to_string(),
      window_title: window_title.to_string(),
    }
  }
}

/// `ActiveWindowV2` is emitted when the active window is changed.
///
/// # Fields
///
/// * `window_address` - Represents the WINDOWADDRESS of the active window.
///
/// # Format
/// activewindow>>WINDOWADDRESS
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveWindowV2 {
  pub window_address: String,
}

impl Parse for ActiveWindowV2 {
  fn parse(data: &str) -> Self {
    ActiveWindowV2 {
      window_address: data.to_string(),
    }
  }
}

/// Emitted when a fullscreen status of a window changes.
///
/// # Fields
///
/// * `enter_fullscreen` - 0/1 (exit fullscreen / enter fullscreen)
///
/// # Format
/// fullscreen>>0/1 (exit fullscreen / enter fullscreen)
#[derive(Debug, Clone, PartialEq)]
pub struct Fullscreen {
  pub enter_fullscreen: bool,
}

impl Parse for Fullscreen {
  fn parse(data: &str) -> Self {
    Fullscreen {
      enter_fullscreen: data == "1",
    }
  }
}

/// Emitted when a monitor is removed (disconnected).
///
/// # Fields
///
/// * `monitor_name` - The name of the monitor that was removed.
///
/// # Format
/// monitorremoved>>MONITORNAME
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorRemoved {
  pub monitor_name: String,
}

impl Parse for MonitorRemoved {
  fn parse(data: &str) -> Self {
    MonitorRemoved {
      monitor_name: data.to_string(),
    }
  }
}

/// Emitted when a monitor is added (connected).
///
/// # Fields
///
/// * `monitor_name` - The name of the monitor that was added.
///
/// # Format
/// monitoradded>>MONITORNAME
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorAdded {
  pub monitor_name: String,
}

impl Parse for MonitorAdded {
  fn parse(data: &str) -> Self {
    MonitorAdded {
      monitor_name: data.to_string(),
    }
  }
}

/// Emitted when a monitor is added (connected).
///
/// # Fields
///
/// * `monitor_id` - The ID of the monitor that was added.
/// * `monitor_name` - The name of the monitor that was added.
/// * `monitor_description` - The description of the monitor that was added.
///
/// # Format
/// monitoraddedv2>>MONITORID,MONITORNAME,MONITORDESCRIPTION
#[derive(Debug, Clone, PartialEq)]
pub struct MonitorAddedV2 {
  pub monitor_id: u32,
  pub monitor_name: String,
  pub monitor_description: String,
}

impl Parse for MonitorAddedV2 {
  fn parse(data: &str) -> Self {
    let parts = data.split(',').collect::<Vec<&str>>();
    if parts.len() != 3 {
      panic!("[MonitorAddedV2::parse()]: Invalid data: {}", data);
    } else {
      let (monitor_id, monitor_name, monitor_description) = (parts[0], parts[1], parts[2]);
      MonitorAddedV2 {
        monitor_id: monitor_id.parse().unwrap(),
        monitor_name: monitor_name.to_string(),
        monitor_description: monitor_description.to_string(),
      }
    }
  }
}

/// `CreateWorkspace` is emitted when a workspace is created.
///
/// # Fields
///
/// * `workspace_name` - Represents the WORKSPACENAME of the created workspace.
///
/// # Format
/// createworkspace>>WORKSPACENAME
#[derive(Debug, Clone, PartialEq)]
pub struct CreateWorkspace {
  pub workspace_name: String,
}

impl Parse for CreateWorkspace {
  fn parse(data: &str) -> Self {
    CreateWorkspace {
      workspace_name: data.to_string(),
    }
  }
}

/// `CreateWorkspaceV2` is emitted when a workspace is created.
///
/// # Fields
///
/// * `workspace_id` - Represents the WORKSPACEID of the created workspace.
/// * `workspace_name` - Represents the WORKSPACENAME of the created workspace.
///
/// # Format
/// createworkspacev2>>WORKSPACEID,WORKSPACENAME
#[derive(Debug, Clone, PartialEq)]
pub struct CreateWorkspaceV2 {
  pub workspace_id: u32,
  pub workspace_name: String,
}

impl Parse for CreateWorkspaceV2 {
  fn parse(data: &str) -> Self {
    let (workspace_id, workspace_name) = data.split_once(",").unwrap();
    CreateWorkspaceV2 {
      workspace_id: workspace_id.parse().unwrap(),
      workspace_name: workspace_name.to_string(),
    }
  }
}

/// `DestroyWorkspace` is emitted when a workspace is destroyed.
///
/// # Fields
///
/// * `workspace_name` - Represents the WORKSPACENAME of the destroyed workspace.
///
/// # Format
/// destroyworkspace>>WORKSPACENAME
#[derive(Debug, Clone, PartialEq)]
pub struct DestroyWorkspace {
  pub workspace_name: String,
}

impl Parse for DestroyWorkspace {
  fn parse(data: &str) -> Self {
    DestroyWorkspace {
      workspace_name: data.to_string(),
    }
  }
}

/// `DestroyWorkspaceV2` is emitted when a workspace is destroyed.
///
/// # Fields
///
/// * `workspace_id` - Represents the WORKSPACEID of the destroyed workspace.
/// * `workspace_name` - Represents the WORKSPACENAME of the destroyed workspace.
///
/// # Format
/// destroyworkspacev2>>WORKSPACEID,WORKSPACENAME
#[derive(Debug, Clone, PartialEq)]
pub struct DestroyWorkspaceV2 {
  pub workspace_id: u32,
  pub workspace_name: String,
}

impl Parse for DestroyWorkspaceV2 {
  fn parse(data: &str) -> Self {
    let (workspace_id, workspace_name) = data.split_once(",").unwrap();
    DestroyWorkspaceV2 {
      workspace_id: workspace_id.parse().unwrap(),
      workspace_name: workspace_name.to_string(),
    }
  }
}

/// `MoveWorkspace` is emitted when a workspace is moved to a different monitor.
///
/// # Fields
///
/// * `workspace_name` - Represents the WORKSPACENAME of the moved workspace.
/// * `monitor_name` - Represents the MONNAME of the monitor the workspace is moved to.
///
/// # Format
/// moveworkspace>>WORKSPACENAME,MONNAME
#[derive(Debug, Clone, PartialEq)]
pub struct MoveWorkspace {
  pub workspace_name: String,
  pub monitor_name: String,
}

impl Parse for MoveWorkspace {
  fn parse(data: &str) -> Self {
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() != 2 {
      panic!("[MoveWorkspace::parse()]: Invalid data: {}", data);
    } else {
      let (workspace_name, monitor_name) = (parts[0], parts[1]);
      MoveWorkspace {
        workspace_name: workspace_name.to_string(),
        monitor_name: monitor_name.to_string(),
      }
    }
  }
}

/// `MoveWorkspaceV2` is emitted when a workspace is moved to a different monitor.
///
/// # Fields
///
/// * `workspace_id` - Represents the WORKSPACEID of the moved workspace.
/// * `workspace_name` - Represents the WORKSPACENAME of the moved workspace.
/// * `monitor_name` - Represents the MONNAME of the monitor the workspace is moved to.
///
/// # Format
/// moveworkspacev2>>WORKSPACEID,WORKSPACENAME,MONNAME
#[derive(Debug, Clone, PartialEq)]
pub struct MoveWorkspaceV2 {
  pub workspace_id: u32,
  pub workspace_name: String,
  pub monitor_name: String,
}

impl Parse for MoveWorkspaceV2 {
  fn parse(data: &str) -> Self {
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() != 3 {
      panic!("[MoveWorkspaceV2::parse()]: Invalid data: {}", data);
    } else {
      let (workspace_id, workspace_name, monitor_name) = (parts[0], parts[1], parts[2]);
      MoveWorkspaceV2 {
        workspace_id: workspace_id.parse().unwrap(),
        workspace_name: workspace_name.to_string(),
        monitor_name: monitor_name.to_string(),
      }
    }
  }
}

/// Emitted when a workspace is renamed.
///
/// # Fields
///
/// * `workspace_id` - The ID of the workspace that was renamed.
/// * `new_name` - The new name of the workspace.
///
/// # Format
/// renameworkspace>>WORKSPACEID,NEWNAME
#[derive(Debug, Clone, PartialEq)]
pub struct RenameWorkspace {
  pub workspace_id: u32,
  pub new_name: String,
}

impl Parse for RenameWorkspace {
  fn parse(data: &str) -> Self {
    let (workspace_id, new_name) = data.split_once(",").unwrap();
    RenameWorkspace {
      workspace_id: workspace_id.parse().unwrap(),
      new_name: new_name.to_string(),
    }
  }
}

/// Emitted when the special workspace opened in a monitor changes (closing results in an empty WORKSPACENAME).
///
/// # Fields
///
/// * `workspace_name` - The name of the workspace.
/// * `monitor_name` - The name of the monitor.
///
/// # Format
/// activespecial>>WORKSPACENAME,MONNAME
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveSpecial {
  pub workspace_name: String,
  pub monitor_name: String,
}

impl Parse for ActiveSpecial {
  fn parse(data: &str) -> Self {
    let (workspace_name, monitor_name) = data.split_once(",").unwrap();
    ActiveSpecial {
      workspace_name: workspace_name.to_string(),
      monitor_name: monitor_name.to_string(),
    }
  }
}

/// Emitted on a layout change of the active keyboard.
///
/// # Fields
///
/// * `keyboard_name` - The name of the keyboard.
/// * `layout_name` - The name of the layout.
///
/// # Format
/// activelayout>>KEYBOARDNAME,LAYOUTNAME
#[derive(Debug, Clone, PartialEq)]
pub struct ActiveLayout {
  pub keyboard_name: String,
  pub layout_name: String,
}

impl Parse for ActiveLayout {
  fn parse(data: &str) -> Self {
    let (keyboard_name, layout_name) = data.split_once(",").unwrap();
    ActiveLayout {
      keyboard_name: keyboard_name.to_string(),
      layout_name: layout_name.to_string(),
    }
  }
}

/// Emitted when a window is opened.
///
/// # Fields
///
/// * `window_address` - The address of the window.
/// * `workspace_name` - The name of the workspace.
/// * `window_class` - The class of the window.
/// * `window_title` - The title of the window.
///
/// # Format
/// openwindow>>WINDOWADDRESS,WORKSPACENAME,WINDOWCLASS,WINDOWTITLE
#[derive(Debug, Clone, PartialEq)]
pub struct OpenWindow {
  pub window_address: String,
  pub workspace_name: String,
  pub window_class: String,
  pub window_title: String,
}

impl Parse for OpenWindow {
  fn parse(data: &str) -> Self {
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() != 4 {
      panic!("[OpenWindow::parse()]: Invalid data: {}", data);
    } else {
      let (window_address, workspace_name, window_class, window_title) =
        (parts[0], parts[1], parts[2], parts[3]);
      OpenWindow {
        window_address: window_address.to_string(),
        workspace_name: workspace_name.to_string(),
        window_class: window_class.to_string(),
        window_title: window_title.to_string(),
      }
    }
  }
}

/// Emitted when a window is closed.
///
/// # Fields
///
/// * `window_address` - The address of the window.
///
/// # Format
/// closewindow>>WINDOWADDRESS
#[derive(Debug, Clone, PartialEq)]
pub struct CloseWindow {
  pub window_address: String,
}

impl Parse for CloseWindow {
  fn parse(data: &str) -> Self {
    CloseWindow {
      window_address: data.to_string(),
    }
  }
}

/// Emitted when a window is moved to a workspace.
///
/// # Fields
///
/// * `window_address` - The address of the window.
/// * `workspace_name` - The name of the workspace.
///
/// # Format
/// movewindow>>WINDOWADDRESS,WORKSPACENAME
#[derive(Debug, Clone, PartialEq)]
pub struct MoveWindow {
  pub window_address: String,
  pub workspace_name: String,
}

impl Parse for MoveWindow {
  fn parse(data: &str) -> Self {
    let (window_address, workspace_name) = data.split_once(",").unwrap();
    MoveWindow {
      window_address: window_address.to_string(),
      workspace_name: workspace_name.to_string(),
    }
  }
}

/// Emitted when a window is moved to a workspace.
///
/// # Fields
///
/// * `window_address` - The address of the window.
/// * `workspace_id` - The ID of the workspace.
/// * `workspace_name` - The name of the workspace.
///
/// # Format
/// movewindowv2>>WINDOWADDRESS,WORKSPACEID,WORKSPACENAME
#[derive(Debug, Clone, PartialEq)]
pub struct MoveWindowV2 {
  pub window_address: String,
  pub workspace_id: u32,
  pub workspace_name: String,
}

impl Parse for MoveWindowV2 {
  fn parse(data: &str) -> Self {
    let parts: Vec<&str> = data.split(',').collect();
    if parts.len() != 3 {
      panic!("[MoveWindowV2::parse()]: Invalid data: {}", data);
    } else {
      let (window_address, workspace_id, workspace_name) = (parts[0], parts[1], parts[2]);
      MoveWindowV2 {
        window_address: window_address.to_string(),
        workspace_id: workspace_id.parse().unwrap(),
        workspace_name: workspace_name.to_string(),
      }
    }
  }
}

/// Emitted when a window title changes.
///
/// # Fields
///
/// * `window_address` - The address of the window whose title has changed.
///
/// # Format
/// windowtitle>>WINDOWADDRESS
#[derive(Debug, Clone, PartialEq)]
pub struct WindowTitle {
  pub window_address: String,
}

impl Parse for WindowTitle {
  fn parse(data: &str) -> Self {
    WindowTitle {
      window_address: data.to_string(),
    }
  }
}

/// Emitted when a layerSurface is mapped.
///
/// # Fields
///
/// * `namespace` - The namespace of the layerSurface.
///
/// # Format
/// openlayer>>NAMESPACE
#[derive(Debug, Clone, PartialEq)]
pub struct OpenLayer {
  pub namespace: String,
}

impl Parse for OpenLayer {
  fn parse(data: &str) -> Self {
    OpenLayer {
      namespace: data.to_string(),
    }
  }
}

/// Emitted when a layerSurface is unmapped.
///
/// # Fields
///
/// * `namespace` - The namespace of the layerSurface.
///
/// # Format
/// closelayer>>NAMESPACE
#[derive(Debug, Clone, PartialEq)]
pub struct CloseLayer {
  pub namespace: String,
}

impl Parse for CloseLayer {
  fn parse(data: &str) -> Self {
    CloseLayer {
      namespace: data.to_string(),
    }
  }
}

/// Emitted when a keybind submap changes. Empty means default.
///
/// # Fields
///
/// * `submap_name` - The name of the submap.
///
/// # Format
/// submap>>SUBMAPNAME
#[derive(Debug, Clone, PartialEq)]
pub struct Submap {
  pub submap_name: String,
}

impl Parse for Submap {
  fn parse(data: &str) -> Self {
    Submap {
      submap_name: data.to_string(),
    }
  }
}

/// Emitted when a window changes its floating mode.
///
/// # Fields
///
/// * `window_address` - The address of the window.
/// * `floating` - Whether the window is in floating mode (`true`) or not (`false`).
///
/// # Format
/// changefloatingmode>>WINDOWADDRESS,FLOATING
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeFloatingMode {
  pub window_address: String,
  pub floating: bool,
}

impl Parse for ChangeFloatingMode {
  fn parse(data: &str) -> Self {
    let (window_address, floating) = data.split_once(",").unwrap();
    ChangeFloatingMode {
      window_address: window_address.to_string(),
      floating: floating == "1",
    }
  }
}

/// Emitted when a window requests an urgent state.
///
/// # Fields
///
/// * `window_address` - The address of the window.
///
/// # Format
/// urgent>>WINDOWADDRESS
#[derive(Debug, Clone, PartialEq)]
pub struct Urgent {
  pub window_address: String,
}

impl Parse for Urgent {
  fn parse(data: &str) -> Self {
    Urgent {
      window_address: data.to_string(),
    }
  }
}

/// Emitted when a window requests a change to its minimized state.
///
/// # Fields
///
/// * `window_address` - The address of the window.
/// * `minimized` - Whether the window is minimized (`true`) or not (`false`).
///
/// # Format
/// minimize>>WINDOWADDRESS,MINIMIZED
#[derive(Debug, Clone, PartialEq)]
pub struct Minimize {
  pub window_address: String,
  pub minimized: bool,
}

impl Parse for Minimize {
  fn parse(data: &str) -> Self {
    let (window_address, minimized) = data.split_once(",").unwrap();
    Minimize {
      window_address: window_address.to_string(),
      minimized: minimized == "1",
    }
  }
}

/// Emitted when a screencopy state of a client changes. Note there might be multiple separate clients.
///
/// # Fields
///
/// * `state` - The screencopy state (`true` for active, `false` for inactive).
/// * `owner` - Indicates the owner of the screencopy session (0 for monitor share, 1 for window share).
///
/// # Format
/// screencast>>STATE,OWNER
#[derive(Debug, Clone, PartialEq)]
pub struct Screencast {
  pub state: bool,
  pub owner: u8, // 0 - monitor share, 1 - window share
}

impl Parse for Screencast {
  fn parse(data: &str) -> Self {
    let (state, owner) = data.split_once(",").unwrap();
    Screencast {
      state: state == "1",
      owner: owner.parse().unwrap(),
    }
  }
}

/// Emitted when ignoregrouplock is toggled.
///
/// # Fields
///
/// * `state` - The new state of ignoregrouplock (`true` for enabled, `false` for disabled).
///
/// # Format
/// ignoregrouplock>>STATE
#[derive(Debug, Clone, PartialEq)]
pub struct IgnoreGroupLock {
  pub state: bool,
}

impl Parse for IgnoreGroupLock {
  fn parse(data: &str) -> Self {
    IgnoreGroupLock { state: data == "1" }
  }
}

/// Emitted when lockgroups is toggled.
///
/// # Fields
///
/// * `state` - The new state of lockgroups (`true` for enabled, `false` for disabled).
///
/// # Format
/// lockgroups>>STATE
#[derive(Debug, Clone, PartialEq)]
pub struct LockGroups {
  pub state: bool,
}

impl Parse for LockGroups {
  fn parse(data: &str) -> Self {
    LockGroups { state: data == "1" }
  }
}

/// Emitted when a window is pinned or unpinned.
///
/// # Fields
///
/// * `window_address` - The address of the window.
/// * `pin_state` - Whether the window is pinned (`true`) or unpinned (`false`).
///
/// # Format
/// pin>>WINDOWADDRESS,PINSTATE
#[derive(Debug, Clone, PartialEq)]
pub struct Pin {
  pub window_address: String,
  pub pin_state: bool,
}

impl Parse for Pin {
  fn parse(data: &str) -> Self {
    let (window_address, pin_state) = data.split_once(",").unwrap();
    Pin {
      window_address: window_address.to_string(),
      pin_state: pin_state == "1",
    }
  }
}

// Note: The `configreloaded` event does not carry specific data according to the provided documentation,
// so no struct is needed for it unless you want to handle it explicitly for consistency or future extension.
