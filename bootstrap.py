#!/usr/bin/env python3

import os
import urllib.request as urlrequest
import subprocess

class WebResource():
    def __init__(self, filename, url, stage_dir):
        self._url = url
        self._stage_dir = stage_dir
        self._filename = filename

    def download(self):
        if not os.path.isdir(self._stage_dir):
            os.makedirs(self._stage_dir)
        if not os.path.exists(os.path.join(self._stage_dir, self._filename)):
            urlrequest.urlretrieve(self._url, 
                os.path.join(self._stage_dir, self._filename))

    def extract(self):
        filepath = os.path.join(self._stage_dir, self._filename)
        extract_tags = { 'bz2' : '-xjf',
                         'gz'  : '-xzf',
                         'tgz' : '-xzf',
                       }
        extract = ''
        for key,value in extract_tags.items():
            if filepath.endswith(key):
                extract = value
                break
        if extract == '':
            raise Exception('Could not extract unknown file extension: ' + 
                            filepath )

        subprocess.check_call(['tar', '-C', self._stage_dir, extract, filepath])

class BuildBoost(WebResource):
    URL = 'http://downloads.sourceforge.net/project/boost/boost/1.57.0/boost_1_57_0.tar.bz2'
    FILENAME = 'boost_1_57_0.tar.bz2'
    DEPS_DIR = os.path.join(os.getcwd(), 'deps')
    BUILD_DIR = os.path.join(DEPS_DIR, 'boost_1_57_0')

    def __init__(self):
        super().__init__(self.FILENAME, self.URL, self.DEPS_DIR)

    def build(self):
        orig_dir = os.getcwd()
        os.chdir(self.BUILD_DIR)
        subprocess.check_call(['sh', 'bootstrap.sh', '--with-python=python3.4'])
        boost_modules = [
            'date_time', 
            'filesystem',
            'log',
            'program_options',
            'python',
            'system',
            'thread']
        b2_build_args = ['link=static', 'variant=release', 'cxxflags=-fPIC',
            'cflags=-fPIC']
        for m in boost_modules:
            b2_build_args += ['--with-'+m]
        subprocess.check_call(['./b2'] + b2_build_args)
        os.chdir(orig_dir)
        os.environ['BOOST_ROOT'] = self.BUILD_DIR

class BuildNanoPB(WebResource):
    URL = {
            'linux':'http://koti.kapsi.fi/~jpa/nanopb/download/nanopb-0.3.1-linux-x86.tar.gz',
          }
    FILENAME = 'nanopb-0.3.1-linux-x86.tar.gz'
    DIRNAME = 'nanopb-0.3.1-linux-x86'
    DEPS_DIR = os.path.join(os.getcwd(), 'deps')

    def __init__(self):
        super().__init__(self.FILENAME, self.URL['linux'], self.DEPS_DIR)
        os.environ['NANOPB_ROOT'] = os.path.join(self.DEPS_DIR, self.DIRNAME)

class BuildLL():
    def __init__(self):
        self.buildDir = os.path.join(os.getcwd(), 'build-liblinkbot')
        self.stageDir = os.path.join(os.getcwd(), 'stage-liblinkbot')
        self.srcDir = os.path.join(os.getcwd(), 'LinkbotLabs-SDK')
        self.toolchainFile = None
        try:
            self.toolchainFile = os.environ['CMAKE_TOOLCHAIN_FILE']
            self.buildDir += '-'+'crosscompile'
            self.stageDir += '-'+'crosscompile'
        except:
            pass
        
    def build(self):
        orig_dir = os.getcwd()
        if not os.path.exists(self.buildDir):
            os.makedirs(self.buildDir)
        if not os.path.exists(self.stageDir):
            os.makedirs(self.stageDir)

        os.chdir(self.buildDir)
        subprocess_args = [
                'cmake', 
                '-G', 'Unix Makefiles', 
                '-DCMAKE_CXX_FLAGS=-fPIC', 
                '-DBUILD_SHARED_LIBS=OFF',
                '-DCMAKE_BUILD_TYPE=Release',
                '-DCMAKE_INSTALL_PREFIX='+self.stageDir,
                ]
        if self.toolchainFile is not None:
            subprocess_args += ['-DCMAKE_TOOLCHAIN_FILE='+self.toolchainFile]
        subprocess_args += [self.srcDir]
        subprocess.check_call(subprocess_args)

        subprocess.check_call(['make', 'baromesh', 'VERBOSE=1'])
        os.chdir('baromesh')
        subprocess.check_call(['make', 'install'])
        os.chdir(os.path.join(self.buildDir, 'ribbon-bridge'))
        subprocess.check_call(['make', 'install'])
        os.chdir(os.path.join(self.buildDir, 'libsfp'))
        subprocess.check_call(['make', 'install'])
        os.chdir(orig_dir)

def main():
    boost = BuildBoost()
    boost.download()
    boost.extract()
    boost.build()

    nanopb = BuildNanoPB()
    nanopb.download()
    nanopb.extract()

    linkbotlabs = BuildLL()
    linkbotlabs.build()

if __name__ == '__main__':
    main()
