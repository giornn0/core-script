use crate::utils::TitleCase;
use crate::Properties;
use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::{
    fs::{create_dir, create_dir_all, File},
    io::Error,
};
pub struct Config {
    section: String,
    section_plural: String,
    api_route: String,
    front_route: String,
    icon: String,
    properties: Properties,
}
impl TitleCase for &str {
    fn title(&self) -> String {
        if !self.is_ascii() || self.is_empty() {
            return String::new();
        }
        let (head, tail) = self.split_at(1);
        head.to_uppercase() + tail
    }
}
impl Config {
    pub fn new(
        section: String,
        section_plural: String,
        api_route: String,
        front_route: String,
        icon: String,
        properties: Properties,
    ) -> Config {
        Config {
            section,
            section_plural,
            api_route,
            front_route,
            icon,
            properties,
        }
    }
}
pub fn create(config: Config) -> Result<(), Error> {
    let base_path = format!("./src/app/views/{}", &config.section_plural);
    let service_path = format!("./src/app/services/http/{}", &config.section_plural);
    let model_name = &config.section.as_str().title();
    let plural_titled = &config.section_plural.as_str().title();
    create_dir_all(&base_path)?;
    create_dir_all(&service_path)?;
    let templates_path = format!("{}/templates", &base_path);
    create_dir(&templates_path)?;
    let form_path = format!("{}/pages/form", &base_path);
    create_dir_all(&form_path)?;
    let list_path = format!("{}/pages/list", &base_path);
    create_dir(&list_path)?;
    let modal_path = format!("{}/pages/modal", &base_path);
    create_dir(&modal_path)?;
    let show_path = format!("{}/pages/show", &base_path);
    create_dir(&show_path)?;
    let mut form_html = File::create(format!("{}/form.component.html", &form_path)).unwrap();
    write_form_html(&mut form_html, &config.section)?;
    let mut form_ts = File::create(format!("{}/form.component.ts", &form_path)).unwrap();
    write_form_ts(&mut form_ts, &config.section, model_name)?;

    let mut list_html = File::create(format!("{}/list.component.html", &list_path)).unwrap();
    write_list_html(&mut list_html, plural_titled, &config.front_route)?;
    let mut list_ts = File::create(format!("{}/list.component.ts", &list_path)).unwrap();
    write_list_ts(&mut list_ts, model_name, &config.section, plural_titled)?;

    let mut show_html = File::create(format!("{}/show.component.html", &show_path)).unwrap();
    write_show_html(&mut show_html)?;
    let mut show_ts = File::create(format!("{}/show.component.ts", &show_path)).unwrap();
    write_show_ts(&mut show_ts, model_name, &config.section)?;

    let mut modal_html = File::create(format!("{}/modal.component.html", &modal_path)).unwrap();
    write_modal_html(&mut modal_html, &config.section)?;
    let mut modal_ts = File::create(format!("{}/modal.component.ts", &modal_path)).unwrap();
    write_modal_ts(&mut modal_ts, model_name, &config.section)?;

    let mut routing = File::create(format!(
        "{}/{}-routing.module.ts",
        &base_path, &config.section_plural
    ))?;
    write_routing(
        &mut routing,
        model_name,
        &config.section_plural,
        plural_titled,
        &config.section,
    )?;

    let mut module = File::create(format!(
        "{}/{}.module.ts",
        &base_path, &config.section_plural
    ))?;
    write_module(&mut module, &config.section_plural, plural_titled)?;

    let mut resolver = File::create(format!(
        "{}/{}.resolver.ts",
        &service_path, &config.section_plural
    ))?;
    write_resolvers(&mut resolver, model_name, plural_titled)?;

    update_nav(
        plural_titled,
        model_name,
        &config.section_plural,
        &config.icon,
    )?;
    update_routes(model_name, plural_titled, &config.section_plural)?;
    update_route_enums(model_name, &config.front_route, &config.api_route)?;

    let mut form_interface = File::create(format!("{}/form.interface.ts", &templates_path))?;
    write_form_interface(&mut form_interface, model_name)?;

    let mut form_template = File::create(format!("{}/form.template.ts", &templates_path))?;
    write_form_template(&mut form_template, &config.properties)?;

    let mut list_template = File::create(format!("{}/list.temp.functions.ts", &templates_path))?;
    write_list_template(
        &mut list_template,
        model_name,
        &config.section,
        &config.icon,
    )?;

    let mut model_ts = File::create(format!(
        "./src/app/shared/models/{}.model.ts",
        &config.section
    ))?;
    write_model_ts(&mut model_ts, model_name, &config.properties)?;
    Ok(())
}

fn write_model_ts(file: &mut File, model: &str, properties: &Properties) -> Result<(), Error> {
    writeln!(
        file,
        r#"export interface {model} {{
  id : number;
  /*
   {:?}
  */
  created_at : Date;
  updated_at: Date;
}}
"#,
        properties
    )
}

fn write_list_template(
    file: &mut File,
    model: &str,
    section: &str,
    icon: &str,
) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ {model} }} from "../../../shared/models/{section}.model";
import {{
  MainListTemplate,
  ListTemplate,
  SpecialBlock,
  ButtonAction,
  Color,
  ButtonType,
}} from "../../../shared/models/basic/template.list.model";
import {{ FontDataRender }} from "../../../shared/models/basic/fonts.template.const";

export const template: MainListTemplate = {{
  headIcon: "fas fa-{icon} fa-2x ms-1 mt-1",
}};

export function setData(obj: {model}): ListTemplate {{
  const  btnDestacado: ButtonAction = {{
    name: obj.id ? "Destacado" : "No destacado",
    icon: obj.id ? "fas fa-star" : '',
    color: obj.id ? Color.warning : Color.medium,
    type: ButtonType.chip,
    tooltip: obj.id ? "No destacar" : "Destacar",
    values: {{ name: "isDestacado", status: obj.id, id: obj.id }},
  }};
  return {{
    nombreForDelete: `${{obj.id}}`,
    avatar: "{icon}",
    alerted:{{
      message: 'Cod. '+obj.id,
      color: Color.primary
    }},
    header: {{ header: "${{obj.id}}", fontEnum: FontDataRender.Header }},
    personalEdit: true,
    personalDelete: true,
    id: obj.id,
    specialButtons: [btnDestacado],
    status: obj.id?`${{obj.id ? "true" : "false"}}`: undefined,
    properties: [],
  }};
}}
export function showSetData(obj: {model}): ListTemplate{{
    return {{  }} as ListTemplate
}}
export function setValueQueryPlus(obj: {model}) {{
  return {{  }};
}}"#
    )
}

fn write_form_template(file: &mut File, properties: &Properties) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Validators }} from "@angular/forms";
import {{
  InputTemplate,
  InputTypes,
}} from "../../../shared/models/basic/template.form.model";
import {{ InitialState }} from "../../../root/root-form/inputs/inputs.component";
/*
{:?}
*/

export const initialState: {{ [x: string]: InitialState }} = {{
  name: {{ value: null, disabled: false , validators: [Validators.required] }},
}};


export const campsTemplate: Array<InputTemplate[]> = [
  [
    {{
      name: "name",
      class: "col-sm-6",
      label: "Nombre Visible",
      required: true,
      typeinput: "text",
    }},
  ]
];"#,
        properties
    )
}

fn write_form_interface(file: &mut File, model: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ FormInterface }} from "../../../root/root-form/form/form.component";

export interface {model}Form extends FormInterface{{
}}
"#
    )
}

fn update_route_enums(model: &str, front_route: &str, api_route: &str) -> Result<(), Error> {
    let mut route_enums = OpenOptions::new()
        .write(true)
        .open("./src/app/constants/routes.ts")?;
    route_enums.seek(SeekFrom::End(-4))?;
    writeln!(
        route_enums,
        r#",
  {model} = "{front_route}",
  {model}Api = "{api_route}"
}}"#
    )
}

fn update_routes(model: &str, plural_titled: &str, section_plural: &str) -> Result<(), Error> {
    let mut routes = OpenOptions::new()
        .write(true)
        .open("./src/app/_routes.ts")?;
    routes.seek(SeekFrom::End(-4))?;
    writeln!(
        routes,
        r#"
  {{
    path: RoutesEnum.{model},
    loadChildren: () =>
      import("./views/{section_plural}/{section_plural}.module").then(
        (m) => m.{plural_titled}Module
      ),
  }},
];"#
    )
}

fn update_nav(
    plural_titled: &str,
    model: &str,
    section_plural: &str,
    icon: &str,
) -> Result<(), Error> {
    let mut nav = OpenOptions::new().write(true).open("./src/app/_nav.ts")?;
    nav.seek(SeekFrom::End(-4))?;
    writeln!(
        nav,
        r#"
  {{
    title: true,
    name: "{plural_titled}",
    class: "strong-font",
    children: [
      {{
        name: "{plural_titled}",
        url: "{section_plural}/listar",
        icon: "fa fa-{icon} scaling",
        class: "strong-font",
        active: RoutesEnum.{model},
      }},
    ],
  }},
];"#
    )
}

fn write_resolvers(file: &mut File, model: &str, plural_titled: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Injectable }} from '@angular/core';
import {{Resolve,
  RouterStateSnapshot,
  ActivatedRouteSnapshot
}} from '@angular/router';
import {{ Observable }} from 'rxjs';
import {{ RoutesEnum }} from '../../../constants/routes';
import {{ RootService }} from '../root.service';

@Injectable({{
  providedIn: 'root'
}})
export class All{plural_titled}Resolver implements Resolve<unknown> {{
  constructor(private rootService:RootService){{}}
  resolve(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Observable<unknown> {{
    return this.rootService.getAll(RoutesEnum.{model}Api)
  }}
}}
@Injectable({{
  providedIn: 'root'
}})
export class {model}Resolver implements Resolve<unknown> {{
  constructor(private rootService:RootService){{}}
  resolve(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Observable<unknown> {{
    const{{id}} = route.params
    return this.rootService.getOne(RoutesEnum.{model}Api,id)
  }}
}}
@Injectable({{
  providedIn: 'root'
}})
export class {plural_titled}Resolver implements Resolve<unknown> {{
  constructor(private rootService:RootService){{}}
  resolve(route: ActivatedRouteSnapshot, state: RouterStateSnapshot): Observable<unknown> {{
    const {{take,page,search}} = route.queryParams
    return this.rootService.index('productos/rubros',page,take,search)
  }}
}}
"#
    )
}

fn write_routing(
    file: &mut File,
    model: &str,
    section_plural: &str,
    plural_titled: &str,
    section: &str,
) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ NgModule }} from "@angular/core";
import {{ Routes, RouterModule }} from "@angular/router";
//import {{ Modal{plural_titled}Resolver }} from "../../constants/resolvers";
import {{ RoutesEnum }} from "../../constants/routes";
import {{
  All{plural_titled}Resolver,
  {model}Resolver,
}} from "../../services/http/{section_plural}/{section_plural}.resolver";
import {{ FormComponent }} from "./pages/form/form.component";
import {{ ListComponent }} from "./pages/list/list.component";
import {{ campsTemplate, initialState }} from "./templates/form.template";
import {{
  template,
  setData,
  setValueQueryPlus,
  showSetData,
}} from "./templates/list.temp.functions";
import {{ ModalComponent }} from "./pages/modal/modal.component";
import {{ ShowComponent }} from "./pages/show/show.component";

const section = RoutesEnum.{model};
const apiSection = RoutesEnum.{model}Api;

const routes: Routes = [
  {{
    path: "",
    data: {{
      title: "{plural_titled}",
    }},
    children: [
      {{
        path: "crear",
        resolve: {{ {section_plural}: All{plural_titled}Resolver }},
        component: FormComponent,
        data: {{
          title: "Nuevo {model}",
          initialState,
          campsTemplate,
          section,
          apiSection,
        }},
      }},
      {{
        path: "listar",
        component: ListComponent,
        data: {{
          title: "{plural_titled}",
          template,
          campsTemplate,
          setData,
          setValueQueryPlus,
          section,
          apiSection,
          initialState,
          modalComponent: ModalComponent,
          showComponent: ShowComponent,
          showSetData,
      //  resolvers:{{
      //    {section_plural}: Modal{plural_titled}Resolver,
      //  }}
        }},
      }},
      {{
        path: ":id/editar",
        component: FormComponent,
        resolve: {{ {section}: {model}Resolver, {section_plural}: All{plural_titled}Resolver }},
        data: {{
          title: "Editar {model}",
          initialState,
          campsTemplate,
          section,
          apiSection,
        }},
      }},
    ],
  }},
];

@NgModule({{
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule],
}})
export class {plural_titled}RoutingModule {{}}
export const routingComponents = [FormComponent, ListComponent, ShowComponent, ModalComponent];
"#
    )
}
fn write_module(file: &mut File, section_plural: &str, plural_titled: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ CommonModule }} from '@angular/common';
import {{ NgModule }} from '@angular/core';
import {{ ReactiveFormsModule, FormsModule }} from '@angular/forms';
import {{ PaginationModule }} from 'ngx-bootstrap/pagination';
import {{ SharedModule }} from '../../shared/shared.module';
import {{ {plural_titled}RoutingModule, routingComponents }} from './{section_plural}-routing.module';
import {{ RootModule }} from '../../root/root.module';
import {{ MatPaginatorModule }} from '@angular/material/paginator';
import {{ HeaderListadoModule }} from '../../shared/components/header-listado/header-listado.module';
import {{ MatDialogModule }} from '@angular/material/dialog';

@NgModule({{
  imports: [
    {plural_titled}RoutingModule,
    ReactiveFormsModule,
    CommonModule,
    PaginationModule,
    FormsModule,
    SharedModule,
    RootModule,
    MatPaginatorModule,
    MatDialogModule,
    HeaderListadoModule,
  ],
  declarations: routingComponents
}})
export class {plural_titled}Module {{ }}
"#
    )
}
fn write_form_html(file: &mut File, section: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"<app-root-form
    #form
    [template]="campsTemplate"
    [title]="title"
    [forEdit]="{}ForEdit"
    [initialStateControls]="initialState"
    [selectOptions]="optionsPassed"
    [section]="section!"
    [APIroute]="apiSection!"
>
</app-root-form>"#,
        section
    )
}
fn write_form_ts(file: &mut File, section: &str, model: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{Component, OnInit, AfterViewInit }} from "@angular/core";
import {{ActivatedRoute, Data }} from "@angular/router";
import {{FormBaseComponent }} from "../../../../root/root-form/form-base/form-base.component";
import {{MatDialog }} from "@angular/material/dialog";
import {{RootService }} from "../../../../services/http/root.service";
import {{InteractionService }} from "../../../../services/interaction/interaction.service";
import {{ {model} }} from "../../../../shared/models/{section}.model";
import {{ {model}Form }} from "../../templates/form.interface";

@Component({{
  templateUrl: "form.component.html",
  styleUrls: ["../../../../app.component.css"],
}})
export class FormComponent extends FormBaseComponent implements {model}Form, OnInit, AfterViewInit {{
  constructor(
    activatedRoute: ActivatedRoute,
    interaction: InteractionService,
    dialog: MatDialog,
    rootService: RootService
  ) {{
    super(activatedRoute, interaction, dialog, rootService);
  }}

  override activoInactivoOptions: any[] = [
    {{value: "destacado", description: "Destacado" }},
    {{value: "estado", description: "Activo" }},
  ];
  {section}ForEdit?: {model};

  ngOnInit() {{ 
    this.activatedRoute.data.subscribe((data) => {{
      this.setStartingValues(data);
    }});
  }}
  setStartingValues(data: Data){{
    const {{ {section} }} = data;
    this.optionsPassed["opciones"] = this.activoInactivoOptions; // Cada Campo usa su name como respectiva key
    this.{section}ForEdit = {section};
    if (this.{section}ForEdit) {{
    // this.{section}ForEdit.opciones = [];
    // if (this.{section}ForEdit?.estado){{
    //   this.{section}ForEdit.opciones.push("estado");
    // }}
    }}
  }}
  ngAfterViewInit(): void {{
    //ACA HACER USO DEL FORM
    this.inputsControls = this.form?.inputs.toArray();
  }}
}}
"#
    )
}

fn write_list_html(file: &mut File, plural_titled: &str, front_route: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"<div #topScrollAnchor></div>
<div class="row wrapp-all animated fadeIn">
  <div class="col-sm-12">
    <div class="row">
      <div class="col-12 col-lg-5 mb-2">
        <label class="form-title pt-0">
          <span class="d-none d-xl-inline"> Listado </span>
          {{{{ title }}}}
          <small *ngIf="listForTemplate?.length">
            {{{{ "(" + itemsPerPage + " de " + totalItems + ")" }}}}
          </small>
        </label>
      </div>

      <div class="col-12 col-lg-7 mb-2">
        <div class="row">
          <div class="col-12 col-sm-6 mb-2">
            <app-shared-searcher (valueSearch)="search($event)">
            </app-shared-searcher>
          </div>
          <div class="col-12 col-sm-6">
            <app-header-listado [redirecTo]="'/{front_route}/crear'"></app-header-listado>
          </div>
        </div>
      </div>

      <div class="col pb-2">
        <app-root-list
          [mainDelete]="true"
          [template]="templ"
          [listado]="listForTemplate"
          [allOpensNoHeader]="true"
          [queryPlus]="queryPlusSet"
          [route]="section!"
          [section]="apiSection!"
        >
        </app-root-list>
      </div>

      <div class="col-12" *ngIf="listado{plural_titled}?.length">
        <mat-paginator
          [length]="totalItems"
          [pageSize]="listado{plural_titled}?.length"
          [pageSizeOptions]="defaultTakeOptions"
          aria-label="Elegir página"
          showFirstLastButtons
          (page)="changePage($event)"
        >
        </mat-paginator>
      </div>
    </div>
  </div>
</div>
"#
    )
}
fn write_list_ts(
    file: &mut File,
    model: &str,
    section: &str,
    plural_titled: &str,
) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Component, OnInit }} from "@angular/core";
import {{ActivatedRoute, Router }} from "@angular/router";
import {{ {model} }} from "../../../../shared/models/{section}.model";
import {{RootService }} from "../../../../services/http/root.service";
import {{ArrayResponse }} from "../../../../shared/models/basic/res.array.model";
import {{ModalService }} from "../../../../services/modals/modals.service";
import {{ListBaseComponent }} from "../../../../root/root-list/list-base/list-base.component";
import {{LoginService }} from "../../../../services/login/login.service";
import {{UserRole }} from "../../../../constants/userRoles";
import {{first }} from "rxjs";
import {{InteractionService }} from "../../../../services/interaction/interaction.service";
import {{MatDialog }} from "@angular/material/dialog";

@Component({{
  selector: "app-list",
  templateUrl: "./list.component.html",
  styleUrls: ["../../../../app.component.css"],
}})
export class ListComponent extends ListBaseComponent implements OnInit {{
  listado{plural_titled}: {model}[] = [];
  constructor(
    private logService: LoginService,
    activatedRoute: ActivatedRoute,
    rootService: RootService,
    router: Router,
    modalService: ModalService,
    interaction: InteractionService,
    dialog: MatDialog,
  ) {{
    super(activatedRoute, router, rootService, modalService, interaction, dialog);
  }}

  userRole?: UserRole;
  userId?: string|null;

  ngOnInit() {{
    // this.logService.getRole.pipe(first()).subscribe(role=>this.userRole = role as UserRole);
    this.logService.getId.pipe(first()).subscribe(id=>{{
      this.userId = id}});
    this.subscriptions.add(
      this.activatedRoute.queryParams.subscribe((query) => {{
        const {{ page, take, search }} = query;

        this.queryParams.queryParams = {{ page, take, search }};
        this.rootService
          .index<{model}>(this.apiSection!, page, take, search,)
          .subscribe((res: ArrayResponse<{model}>) => {{
            this.listForTemplate = [];
            this.listado{plural_titled} = res.data;
            this.listado{plural_titled}?.forEach(({section}) => {{
              this.listForTemplate.push(this.setData({section}, Number(this.userId)));
              this.queryPlusSet.push(this.setValueQueryPlus({section}));
            }});
            this.setPage(res);
          }});
      }})
    );
  }}
}}
"#
    )
}
fn write_show_html(file: &mut File) -> Result<(), Error> {
    writeln!(
        file,
        r#"<app-root-show
    [forShow]="showing"
>
</app-root-show>
"#
    )
}
fn write_show_ts(file: &mut File, model: &str, section: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Component, Inject }} from "@angular/core";
import {{ MAT_DIALOG_DATA }} from "@angular/material/dialog";
import {{ ModalShowBaseComponent }} from "../../../../root/root-show/modal-show-base/modal-show-base.component";
import {{ DataModalShow }} from "../../../../shared/models/basic/data.routes.model";
import {{ {model} }} from "../../../../shared/models/{section}.model";

@Component({{
    templateUrl: "./show.component.html",
  }})
  export class ShowComponent extends ModalShowBaseComponent {{
    constructor(
    @Inject(MAT_DIALOG_DATA) data: DataModalShow<{model}>
    ){{
        super(data)
    }}
  }}
"#
    )
}
fn write_modal_html(file: &mut File, section: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"<mat-dialog-content>
    <app-root-form 
        #form
        [template]="campsTemplate!"
        [title]="title"
        [forEdit]="{section}ForEdit"
        [initialStateControls]="initialState"
        [selectOptions]="optionsPassed"
        [section]="section!"
        [APIroute]="apiSection!"
        [isModal]="true"
        (close)="close($event)"
    >
    </app-root-form>
</mat-dialog-content>"#
    )
}
fn write_modal_ts(file: &mut File, model: &str, section: &str) -> Result<(), Error> {
    writeln!(
        file,
        r#"import {{ Component, Inject, OnInit, AfterViewInit }} from "@angular/core";
import {{ ChangeDetectorRef }} from "@angular/core";
import {{ LoginService }} from "../../../../services/login/login.service";
import {{ MatDialog, MatDialogRef, MAT_DIALOG_DATA }} from "@angular/material/dialog";
import {{ ModalFormBaseComponent }} from "../../../../root/root-form/modal-form-base/modal-form-base.component";
import {{ DataEditForm }} from "../../../../shared/models/basic/data.routes.model";
import {{ RootService }} from "../../../../services/http/root.service";
import {{ InteractionService }} from "../../../../services/interaction/interaction.service";
import {{ {model} }} from "../../../../shared/models/{section}.model";
import {{ {model}Form }} from "../../templates/form.interface";

@Component({{
  templateUrl: "modal.component.html",
  styleUrls: ["../../../../app.component.css"],
}})
export class ModalComponent extends ModalFormBaseComponent implements {model}Form, OnInit, AfterViewInit {{
  constructor(
    private cdr: ChangeDetectorRef,
    dialogRef: MatDialogRef<ModalComponent>,
    logService: LoginService,
    interaction: InteractionService,
    dialog: MatDialog,
    rootService: RootService,
    @Inject(MAT_DIALOG_DATA) data: DataEditForm
  ) {{
    super(dialogRef, logService, data, interaction, dialog, rootService);
  }}
  override activoInactivoOptions: any[] = [
{{value: "destacado", description: "Destacado" }},
{{value: "estado", description: "Activo" }},
  ];
  
{section}ForEdit?: {model};

  ngOnInit() {{
    this.setStartingValues();
  }}

  setStartingValues(){{
    // const {{ {section} }} = this.data.resolvers!;
    this.optionsPassed["opciones"] = this.activoInactivoOptions; // Cada Campo usa su name como respectiva key
    this.{section}ForEdit = this.data.entityForEdit;
    if (this.{section}ForEdit) {{
    // this.{section}ForEdit.opciones = [];
    // if (this.{section}ForEdit?.estado){{
    //   this.{section}ForEdit.opciones.push("estado");
    // }}
    }}
  }}
  
  ngAfterViewInit(): void {{
    //ACA HACER USO DEL FORM
    this.inputsControls = this.form?.inputs.toArray();
  }}
}}
"#
    )
}
