![Splitter](https://github.com/user-attachments/assets/e57ad76e-97a9-4ff7-a0fa-014777f0dd40)

# Splitter

Splitter is a fast, lightweight, and reliable tool for splitting large datasets into manageable chunks. Designed for efficiency, it can handle files with millions of entries. Splitter supports the most common data types.
Its high performance, powered by Rust, makes it ideal for developers, data analysts, and ML engineers who need a dependable solution for large data preprocessing.

## Main Features

### 1. Speed  
Splitter is fast as it is written in Rust, leveraging the speed and security provided by the language. It handles large datasets efficiently, ensuring minimal processing time even for files with millions of entries.

### 2. File Types  
Splitter supports various file formats, including:  
- **CSV** 
- **JSON**
- **TXT**

### 3. Custom Split Sizes  
Users can specify split sizes in terms of:  
- File size (e.g., split into 100MB chunks).  

### 4. Error Handling  
Robust error handling ensures that malformed files or missing data are flagged, providing detailed error messages to help users debug their datasets.

### 5. Output Customization  
Users can customize output dataset folder names to suit organizational workflows.

### 6. Simple CLI Interface  
Splitter features a user-friendly command-line interface with intuitive commands and flags.

# Installation

You can install Splitter using wget:
```sh
  wget https://github.com/yousef-rafat/Splitter/blob/main/exe/Splitter.exe
```
Alternatively, you can manually download it from the exe folder.

# User Guide

Splitter is simple to interact. A full example of using Splitter:

```sh
Splitter -name animal_dataset -file [FILE-PATH] -size [SIZE_IN_MEGABYTES]
```

If you want to split the file into halves, you could use -half instead of -size. This is supported for all the file formats.

# License
Splitter is under MIT License.
