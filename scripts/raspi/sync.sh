#!/bin/bash

# Copy script

#rsync -r ./scripts/raspi/create_user.sh temp@thermal-camera:~/thermal-camera/
#rsync ./scripts/raspi/config.sh temp@thermal-camera:~/thermal-camera/
#ssh temp@thermal-camera 'cd thermal-camera && ./create_user.sh'

rsync -r --delete ./scripts/raspi pi@thermal-camera:~/thermal-camera-config/
ssh -t pi@thermal-camera 'cd thermal-camera-config ;/bin/bash'
