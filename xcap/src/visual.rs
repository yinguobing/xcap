use rerun::RecordingStream;

pub trait Visual {
    type VisualizeError;

    /// Function to visualize the data with Rerun.
    fn visualize(
        &self,
        entity_path: &str,
        rec: &RecordingStream,
    ) -> Result<(), Self::VisualizeError>;
}
