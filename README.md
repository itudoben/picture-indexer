# Abstract
This project is about to index picture extracted metadata in Elasticsearch.

# Command line
To execute the project against a picture, run this.
```bash
cargo run /Users/jhujol/Projects/rust/exif-rs/tests/exif.jpg
```

This picture fails with the library provided:
```bash
cargo run /Users/jhujol/Desktop/ford_taunus_tc_1972_03.jpeg
```

Picture formats[1] like JPEG, GIF, TIFF and PNG, have metadata stored in EXIF, IPTC, XMP[2] or in textual information chunks for PNG[3] and this can be used to index pictures.

This enables looking for duplicate files as some picture viewer may copy the file and rename it for its purpose therefore creating multiple copies of the same file. By comparing some of the metadata one  or an automated tool could identify these duplicate pictures.
Another improvement once the pictures are indexed is to allow for classification depending on the year, month and date amongst labels or tags available from the metadata.

# Requirements
I would like to index images from the file system directories and visualize in Federate on a timeline when the pictures have been taken. I would like to filter by location or other metadata available from the pictures.

# Technical Details
- Benchmark time to process, CPU usage and memory dump results into Elasticsearch
    - format of what benchmark dumps (Criterion)[5]
- Support for JPEG and PNG formats
- Extract EXIF metadata from JPEG and PNG when possible, and textual information chunks from PNG
- Stored into a temp directory so it can be post processed into Elasticsearch

A test can be made with some websites[4] that can list all EXIF information after uploading a picture to check for the accuracy of the extracted metadata.

# Steps
- End-to-end solution for one JPEG file
    - read the EXIF metadata from a picture using crate kamadak-exif[6]
    - store picture info into JSON file temp dir
    - load the JSON content into Elasticsearch

# References
- [1] https://dev.exiv2.org/projects/exiv2/wiki/The_Metadata_in_PNG_files#References
- [2] https://exiftool.org/TagNames/XMP.html
- [3] https://dev.exiv2.org/projects/exiv2/wiki/The_Metadata_in_PNG_files
- [4] https://www.metadata2go.com/
- [5] https://nickb.dev/blog/guidelines-on-benchmarking-and-rust
- [6] https://docs.rs/kamadak-exif/0.5.4/exif/
