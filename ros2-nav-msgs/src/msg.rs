use ros2_builtin_interfaces::msg::Time;
use ros2_geometry_msgs::msg::{Point, Pose, PoseStamped, PoseWithCovariance, TwistWithCovariance};
use ros2_std_msgs::msg::Header;
use serde::Deserialize;

// An array of navigation goals
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Goals {
    // This header will store the time at which the poses were computed (not to be confused with the stamps of the poses themselves).
    //  In the case that individual poses do not have their frame_id set or their timetamp set they will use the default value here.
    header: Header,

    // An array of goals to for navigation to achieve.
    // The goals should be executed in the order of the array.
    // The header and stamp are intended to be used for computing the position of the goals.
    // They may vary to support cases of goals that are moving with respect to the robot.
    goals: Vec<PoseStamped>,
}

// An array of cells in a 2D grid
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct GridCells {
    header: Header,

    //  Width of each cell
    cell_width: f32,

    // Height of each cell
    cell_height: f32,

    // Each cell is represented by the Point at the center of the cell
    cells: Vec<Point>,
}

// This hold basic information about the characteristics of the OccupancyGrid
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct MapMetaData {
    // The time at which the map was loaded
    map_load_time: Time,

    // The map resolution [m/cell]
    resolution: f32,

    // Map width [cells]
    width: u32,

    // Map height [cells]
    height: u32,

    // The origin of the map [m, m, rad].  This is the real-world pose of the
    // bottom left corner of cell (0,0) in the map.
    origin: Pose,
}

// This represents a 2-D grid map
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct OccupancyGrid {
    header: Header,

    // MetaData for the map
    info: MapMetaData,

    // The map data, in row-major order, starting with (0,0).
    // Cell (1, 0) will be listed second, representing the next cell in the x direction.
    // Cell (0, 1) will be at the index equal to info.width, followed by (1, 1).
    // The values inside are application dependent, but frequently,
    // 0 represents unoccupied, 1 represents definitely occupied, and
    // -1 represents unknown.
    data: Vec<i8>,
}

// This represents an estimate of a position and velocity in free space.
// The pose in this message should be specified in the coordinate frame given by header.frame_id
// The twist in this message should be specified in the coordinate frame given by the child_frame_id
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Odometry {
    // Includes the frame id of the pose parent.
    header: Header,

    // Frame id the pose points to. The twist is in this coordinate frame.
    child_frame_id: String,

    // Estimated pose that is typically relative to a fixed world frame.
    pose: PoseWithCovariance,

    // Estimated linear and angular velocity relative to child_frame_id.
    twist: TwistWithCovariance,
}

// An array of poses that represents a Path for a robot to follow.
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Path {
    // Indicates the frame_id of the path.
    header: Header,

    // Array of poses to follow.
    poses: Vec<PoseStamped>,
}
