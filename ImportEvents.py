#FILENAME:ImportEvents.py
#AUTHOR:Jonathan Shambaugh
#PURPOSE: To extract the comments given in a Toyopuc project and write them to the corresponding address in the template for easy event importing.
#NOTES: See the github repository for more info. https://github.com/jkernal/EDC_Events_tool
#VERSION: v1.1.0
#START DATE: 17 Oct 22

from sys import executable, version
from subprocess import check_call, check_output
from os import system, getcwd, listdir, access, R_OK, W_OK, X_OK
from csv import reader
from shutil import copy
from time import perf_counter
from concurrent.futures import ThreadPoolExecutor



ansi = {
    "Black": "\u001b[30m",
    "Red": "\u001b[31m",
    "Green": "\u001b[32m",
    "Yellow": "\u001b[33m",
    "Blue": "\u001b[34m",
    "Magenta": "\u001b[35m",
    "Cyan": "\u001b[36m",
    "White": "\u001b[37m",
    "Bright Black": "\u001b[30;1m",
    "Bright Red": "\u001b[31;1m",
    "Bright Green": "\u001b[32;1m",
    "Bright Yellow": "\u001b[33;1m",
    "Bright Blue": "\u001b[34;1m",
    "Bright Magenta": "\u001b[35;1m",
    "Bright Cyan": "\u001b[36;1m",
    "Bright White": "\u001b[37;1m",
    "Bold": "\u001b[1m",
    "Underline": "\u001b[4m",
    "Reset": "\u001b[0m"
}



update_check_enabled = True



debug_enabled = False



v = "v1.1.0"



t1 = perf_counter()



def install_lib(lib):
    
    """
    Installs the specified Python library using pip.
    
    Parameters:
        lib (str): The name of the library to install.
    """
    
    print(f"{ansi['Yellow']}\nInstalling {lib}...")
    # implement pip as a subprocess:
    check_call([executable, '-m', 'pip', 'install', lib])

    # process output with an API in the subprocess module:
    requests = check_output([executable, '-m', 'pip','freeze'])
    installed_packages = [r.decode().split('==')[0] for r in requests.split()]

    print(installed_packages)

#check if libraries are installed, if not, install it
try:
    from openpyxl import load_workbook
except ModuleNotFoundError:
    print(f"{ansi['Bright Red']}Openpyxl library is not installed.")
    install_lib("Openpyxl")
    from openpyxl import load_workbook
try:
    from requests import get
except ModuleNotFoundError:
    print(f"{ansi['Bright Red']}Requests library is not installed.")
    install_lib("requests")
    from requests import get
try:
    from tqdm import tqdm
except ModuleNotFoundError:
    print(f"{ansi['Bright Red']}tqdm library is not installed.")
    install_lib("tqdm")
    from tqdm import tqdm



def preamble():
    
    """
    Displays the script title, version, and optionally checks Python version and 
    whether a newer version of the script is available on GitHub.
    """
    
    system('color')
    print(f"{ansi['Underline']}{ansi['Bright Magenta']}Events Layout Import Tool{ansi['Reset']}")
    print(v)
    
    if debug_enabled:
        if version[:4] != "3.10":
            print(f"{ansi['Bright Yellow']}***Warning: The version of Python is different from what this script was written on.***{ansi['Reset']}")
            print("\u001b[37m\u001b[0mPython Version: " + version[:7])
    
    if update_check_enabled:
        owner = "jkernal"
        repo = "EDC_Events_tool"
        print("Checking for updates...", end="",flush=True)
        try:
            response = get(f"https://api.github.com/repos/{owner}/{repo}/releases/latest")
            #print(response.json())
            print("[DONE]")
            if v != response.json()["tag_name"]:
                print(f"{ansi['Bright Yellow']}***Warning: There is a new release of this tool.***\nGo to: https://github.com/jkernal/EDC_Events_tool")
        except:
            print("[FAILED]")
            print("\u001b[33;1m***Warning: Could not connect to repository. Version check failed.***")
        #print(environ)



def manages_files():
    
    """
    Confirms the presence of template, input, and output directories and files.
    Copies the template file into the output directory for modification.

    Returns:
        list: File paths for the template, output copy, and input file.
    """
    
    wrk_dir = getcwd()
    temp_dir, out_dir, in_dir = wrk_dir + '//template', wrk_dir + '//output', wrk_dir + '//input'
    #confirming files
    try:
        temp_loc = temp_dir + '//' + listdir(temp_dir)[0]
    except FileNotFoundError:
        print("\n")
        print(f"{ansi['Bold']}{ansi['Bright Red']}The template directory was not found.\n\nPlease add the template directory and restart.")
        done()
    except IndexError:
        print("\n")
        print(f"{ansi['Bold']}{ansi['Bright Red']}The template file was not found.\n\nPlease add the template file to the template directory and restart.")
        done()
    try:
        in_loc = in_dir + '//' + listdir(in_dir)[0]
    except FileNotFoundError:
        print("\n")
        print(f"{ansi['Bold']}{ansi['Bright Red']}The input file or directory was not found.\n\nPlease add the input file to the input directory and restart.")
        done()
    except IndexError:
        print("\n")
        print(f"{ansi['Bold']}{ansi['Bright Red']}The input file was not found.\n\nPlease add the input file to the input directory and restart.")
        done()
    #Copying template file to output directory
    try:
        copy(temp_loc, out_dir + '//out_' + listdir(temp_dir)[0])
    except FileNotFoundError:
        print("\n")
        print(f"{ansi['Bold']}{ansi['Bright Red']}The output directory was not found.\n\nPlease add the output directory and restart.")
        done()
    except Exception as e:
        print(f"{ansi['Bold']}{ansi['Bright Red']}Make sure to close the template file or make sure template file is not being used by another program.")
        print(e)
        done()


    out_loc = out_dir + '//' + listdir(out_dir)[0]
    locations = [temp_loc, out_loc, in_loc]
    return locations



def perm_check(locs):
    
    """
    Checks read, write, and execute permissions for a list of files.
    
    Parameters:
        locs (list): List of file paths to check.
    """
    
    file_names = ["template", "output", "input"]
    access_type = ["read", "write", "execute"]
    for i in range(len(locs)):
        permissions = [access(locs[i], R_OK), access(locs[i], W_OK), access(locs[i], X_OK)]
        for j in range(len(permissions)):
            if not permissions[j]:
                print(f"{ansi['Bold']}{ansi['Bright Red']}The script does not have {access_type[j]} access to the {file_names[i]} file. Make sure the file is closed and permissions are set.")
            else:
                #print(f"\u001b[1m\u001b[31;1mThe script does have {access_type[j]} access to the {file_names[i]} file. Make sure the file is closed and permissions are set.")
                continue
        continue
    return None



def get_address_array_from_temp(sheet):
    
    """
    Extracts address values from column B of the Excel worksheet starting from row 3.
    
    Parameters:
        sheet (Worksheet): An openpyxl worksheet object.

    Returns:
        list: A list of [address, row number] pairs.
    """
    
    array = []
    for i in range(sheet.max_row):
        array.append([sheet.cell(i+3,2).value, i + 3])
    return array



def get_address_comment_array_from_input(location):
    
    """
    Parses the input CSV file and creates a dictionary of address-comment pairs.

    Parameters:
        location (str): File path of the input CSV.

    Returns:
        dict: Dictionary of address (key) to comment (value).
    """
    
    try:
        array = list(reader(open(location, encoding= "ISO8859")))
    except PermissionError:
        print(f"{ansi['Bold']}{ansi['Bright Red']}Error: Could not access input file.")
        done()
    except:
        print(f"""{ansi['Bright Red']}{ansi['Bold']}An error occurred while reading the Toyopuc Comment file.\n\n
            Possible Causes:\n
            -Too many fields in the file (max 131072)
            -The data is not supported under 'ISO8859' encoding
            -The file is in use""")
    _dict = {}
    for row in array:
        _dict.update({row[0]: row[1]})
    return _dict



def done():
    
    """
    Resets terminal text formatting and exits the program after printing execution time.
    """
    
    print("\u001b[37m\u001b[0m")
    time_elapsed = round((perf_counter() - t1), 3)
    print(" ".join([f"{ansi['Reset']}Execution time: ", f"{time_elapsed}", "sec(s)"]))
    exit()



def main():
    
    """
    Main execution flow:
    - Initializes script
    - Confirms and copies files
    - Checks file permissions
    - Extracts addresses from Excel and comments from CSV
    - Matches and writes comments to the Excel output
    - Saves file and reports results
    """
    
    #run preamble
    preamble()

    #find file locations
    file_locs = manages_files()#file_locs[template, output, input]

    #check permissions on files
    perm_check(file_locs)

    #open output workbook and worksheet
    wb = load_workbook(filename=file_locs[1])
    ws = wb["Import Cheat Sheet"]

    #get address that need comments from template and get addresses with comments from input in separate threads
    with ThreadPoolExecutor() as executor:
        f1 = executor.submit(get_address_array_from_temp, ws)
        f2 = executor.submit(get_address_comment_array_from_input, file_locs[2])
    #wait for results from both threads
    address_array = f1.result()
    address_comment_dict = f2.result()
    #close executor
    executor.shutdown()

    match_count, address_array_len = 0, len(address_array)
    
    print(f"\n{ansi['Reset']}{ansi['Green']}Working on it...",flush=True, end="")

    #loop through the addresses and compare to the array with comments
    for i in tqdm(range(address_array_len)):
        if address_array[i][0] == None:
            continue
        elif len(address_array[i][0]) == 4:
            searched_address = "P1-" + address_array[i][0]
        else:
            searched_address = address_array[i][0]

        if searched_address in address_comment_dict:
            ws.cell(row=address_array[i][1], column=6).value = searched_address
            ws.cell(row=address_array[i][1], column=7).value = address_comment_dict.get(searched_address)
            match_count+=1
        else:
            pass
            
    """     for address in address_comment_array:
            if address_array[i][0] == address[0]:
                ws.cell(row=address_array[i][1], column=6).value = address[0]
                ws.cell(row=address_array[i][1], column=7).value = address[1]
                match_count+=1
            elif address[0][:4] == "P1-X":
                if address_array[i][0] == address[0][3:]:
                    ws.cell(row=address_array[i][1], column=6).value = address[0][3:]
                    ws.cell(row=address_array[i][1], column=7).value = address[1]
                    match_count+=1
                else:
                    continue
            elif address[0][:4] == "P2-D":
                if address_array[i][0] == address[0][3:]:
                    ws.cell(row=address_array[i][1], column=6).value = address[0][3:]
                    ws.cell(row=address_array[i][1], column=7).value = address[1]
                    match_count+=1
                else:
                    continue
            else:
                address_prog_bar.prog = i """
    
    #save changes to the output file
    wb.save(file_locs[1])
    
    #display stats and warning if needed
    print("\nDone.", flush=True)
    print(f"\n{ansi['Bright Blue']}Number of comments found:{ansi['Yellow']}" + str(match_count))
    if match_count == 0:
        print(f"{ansi['Bright Yellow']}***No matches were found. Make sure your input and template files are correct***")

    #reset and exit()
    done()
    #end of main


#entry point
if __name__ == "__main__":
    main()