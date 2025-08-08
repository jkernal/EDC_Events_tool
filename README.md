# EDC-Events-Tool
&nbsp;&nbsp;
## From the author:
I am not a full-time dev. So with that being said there could be errors or bugs that I did not foresee. If any errors or bugs occur that you cannot resolve, please    send me a screenshot of the error or a detailed description of the bug. Thanks and good luck.


## Installation
### Step 1
&nbsp;&nbsp;Make sure you have Python installed AND added to PATH!
![image](https://github.com/user-attachments/assets/13f73752-ffab-4f4d-b469-d7f6d0d274b7)
Complete the Python install.
### Step 2:
&nbsp;&nbsp;Go to releases
![image](https://github.com/user-attachments/assets/a2316742-d4bd-433b-9cd4-b249b9344e53)
Download the latest release.
![image](https://github.com/user-attachments/assets/e3d0b5c4-27da-4a67-af0c-abb9f693dea2)
Extract and put the folder in whatever place you like.

## Using the Tool
Ensure that the .py file has its own directory(folder).
##### Inside the folder the .py file is located in, you will need 4 more directories(folders):
- Toyopuc
- Screenworks
- Output
- Template
  
Ensure folder names are exactly as stated above, otherwise errors will occur.(The reason for doing this is so the file names will not matter)
The Toyopuc folder will contain your comment file from the PcWin project, this should be a csv.
The Screenworks folder will contain you exported Screenworks project ***Make sure to export as Unicode!***
The template folder will contain the template excel file for importing events. **YOU CAN CHANGE THE ADDRESSES IN THIS FILE IF NEEDED.**
##### Running the tool:
You can either run the 'ImportEvents.bat' or the 'ImportEvents.py'. The only difference between the two is the .bat file maintains a terminal after the script completes so if errors do occur you can read them, the .py file will close the terminal when it encounters an error.
The output folder will contain a copy of the template file with all the comments that it could find. You will most likely need to edit some comments to get them to less than 40 characters per EDC requirements.
## Tips and Notes
- Confirm that the comment file you choose out of the Toyopuc project is the english one.
- Sometimes you can receive an error if the comment file is too large, this is an encoding limitation, so a workaround is to open the comment file and delete some rows that you know are not needed.
- If you have any issues with dependencies or library imports let me know.
- When running for the first time it will need to install libraries so don't panic.
## To-Do
- Sit back and relax

  
 
  
