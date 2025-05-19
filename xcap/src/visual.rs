use rerun::RecordingStream;

pub trait Visual {
    type VisualizeError;

    /// Function to visualize the data with Rerun.
    fn visualize(&self, rec: &RecordingStream) -> Result<(), Self::VisualizeError>;
}
