use rust_web_ui::*;

pub struct Example3;

pub struct Example3Props {}

impl Default for Example3Props {
	fn default() -> Example3Props {
		Example3Props { }
	}
}

impl Widget<'_> for Example3 {
	type Props = Example3Props;

	fn render(mut ctx: Ctx<'_>, props: Example3Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Color(Color::Fg(11))]);
			props.set_attribute("style", "min-height: 192px");
			
			props.child("1", Dynamic).run("nav", |props| {
				props.set_attribute("x-data", "{ open: false }");
				props.styles(&[Style::Color(Color::Fg(89))]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(8), Screen::Small(&[Style::PaddingX(24)]), Screen::Large(&[Style::PaddingX(32)])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("relative"), Style::Flex, Style::Width(64), Style::ItemsCenter, Style::JustifyBetween]);
						
						props.child("1", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("left-0"), Style::Flex, Style::ItemsCenter, Screen::Small(&[NoStyle::Noop("hidden")])]);
							
							//  Mobile menu button
							props.child("3", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[NoStyle::Noop("relative"), Style::InlineFlex, Style::ItemsCenter, Style::JustifyCenter, NoStyle::Noop("rounded-md"), Style::Padding(8), Style::TextColor(Color::Fg(44)), Action::Hover(&[Style::Color(Color::Fg(78))]), Action::Hover(&[NoStyle::Noop("text-white")]), Action::Hover(&[NoStyle::Noop("outline-none")]), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-inset")]), Action::Hover(&[NoStyle::Noop("ring-white")])]);
								props.set_attribute("aria-controls", "mobile-menu");
								props.set_attribute("@click", "open = !open");
								props.set_attribute("aria-expanded", "false");
								props.set_attribute("x-bind:aria-expanded", "open.toString()");
								
								props.child("1", Dynamic).run("span", |props| {
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("-inset-0.5")]);
								});
								props.child("3", Dynamic).run("span", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("Open main menu"));
								});
								props.child("5", Icon).run(|props| {
									props.style(&[Style::Block, Style::Width(24), Style::Width(24)]);
								});
								props.child("7", Icon).run(|props| {
									props.style(&[NoStyle::Noop("hidden"), Style::Width(24), Style::Width(24)]);
								});
							});
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::Flex, NoStyle::Noop("flex-1"), Style::ItemsCenter, Style::JustifyCenter, Screen::Small(&[NoStyle::Noop("items-stretch")]), Screen::Small(&[NoStyle::Noop("justify-start")])]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::Flex, NoStyle::Noop("flex-shrink-0"), Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("img", |props| {
									props.styles(&[Style::Width(32), NoStyle::Noop("w-auto")]);
									props.set_attribute("src", "https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=500");
									props.set_attribute("alt", "Your Company");
								});
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[NoStyle::Noop("hidden"), Screen::Small(&[Style::MarginLeft(24)]), Screen::Small(&[Style::Block])]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::SpaceX(16)]);
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Color(Color::Fg(100)), NoStyle::Noop("text-white"), NoStyle::Noop("rounded-md"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium")]);
										props.set_attribute("x-state:on", "Current");
										props.set_attribute("x-state:off", "Default");
										props.set_attribute("aria-current", "page");
										props.set_attribute("x-state-description", r#"Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
										
										props.child("0", Label).run(|props| props.set_text("Dashboard"));
									});
									props.child("3", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::TextColor(Color::Fg(33)), Action::Hover(&[Style::Color(Color::Fg(78))]), Action::Hover(&[NoStyle::Noop("text-white")]), NoStyle::Noop("rounded-md"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium")]);
										props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
										
										props.child("0", Label).run(|props| props.set_text("Team"));
									});
									props.child("5", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::TextColor(Color::Fg(33)), Action::Hover(&[Style::Color(Color::Fg(78))]), Action::Hover(&[NoStyle::Noop("text-white")]), NoStyle::Noop("rounded-md"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium")]);
										props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
										
										props.child("0", Label).run(|props| props.set_text("Projects"));
									});
									props.child("7", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::TextColor(Color::Fg(33)), Action::Hover(&[Style::Color(Color::Fg(78))]), Action::Hover(&[NoStyle::Noop("text-white")]), NoStyle::Noop("rounded-md"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium")]);
										props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
										
										props.child("0", Label).run(|props| props.set_text("Calendar"));
									});
								});
							});
						});
						props.child("5", Dynamic).run("div", |props| {
							props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(8), Screen::Small(&[NoStyle::Noop("static")]), Screen::Small(&[NoStyle::Noop("inset-auto")]), Screen::Small(&[Style::MarginLeft(24)]), Screen::Small(&[Style::PaddingRight(0)])]);
							
							props.child("1", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(89)), Style::Padding(4), Style::TextColor(Color::Fg(44)), Action::Hover(&[NoStyle::Noop("text-white")]), Action::Hover(&[NoStyle::Noop("outline-none")]), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-white")]), Action::Hover(&[NoStyle::Noop("ring-offset-2")]), Action::Hover(&[NoStyle::Noop("ring-offset-gray-800")])]);
								
								props.child("1", Dynamic).run("span", |props| {
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("-inset-1.5")]);
								});
								props.child("3", Dynamic).run("span", |props| {
									props.styles(&[NoStyle::Noop("sr-only")]);
									
									props.child("0", Label).run(|props| props.set_text("View notifications"));
								});
								props.child("5", Icon).run(|props| {
									props.style(&[Style::Width(24), Style::Width(24)]);
								});
							});
							//  Profile dropdown 
							props.child("5", Dynamic).run("div", |props| {
								props.set_attribute("x-data", "Components.menu({ open: false })");
								props.set_attribute("x-init", "init()");
								props.set_attribute("@keydown.escape.stop", "open = false; focusButton()");
								props.set_attribute("@click.away", "onClickAway($event)");
								props.styles(&[NoStyle::Noop("relative"), Style::MarginLeft(12)]);
								
								props.child("1", Dynamic).run("div", |props| {
									props.child("1", Dynamic).run("button", |props| {
										props.set_attribute("type", "button");
										props.styles(&[NoStyle::Noop("relative"), Style::Flex, NoStyle::Noop("rounded-full"), Style::Color(Color::Fg(89)), NoStyle::Noop("text-sm"), Action::Hover(&[NoStyle::Noop("outline-none")]), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-white")]), Action::Hover(&[NoStyle::Noop("ring-offset-2")]), Action::Hover(&[NoStyle::Noop("ring-offset-gray-800")])]);
										props.set_attribute("id", "user-menu-button");
										props.set_attribute("x-ref", "button");
										props.set_attribute("@click", "onButtonClick()");
										props.set_attribute("@keyup.space.prevent", "onButtonEnter()");
										props.set_attribute("@keydown.enter.prevent", "onButtonEnter()");
										props.set_attribute("aria-expanded", "false");
										props.set_attribute("aria-haspopup", "true");
										props.set_attribute("x-bind:aria-expanded", "open.toString()");
										props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
										props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
										
										props.child("1", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("-inset-1.5")]);
										});
										props.child("3", Dynamic).run("span", |props| {
											props.styles(&[NoStyle::Noop("sr-only")]);
											
											props.child("0", Label).run(|props| props.set_text("Open user menu"));
										});
										props.child("5", Dynamic).run("img", |props| {
											props.styles(&[Style::Width(32), Style::Width(32), NoStyle::Noop("rounded-full")]);
											props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
											props.set_attribute("alt", "");
										});
									});
								});
								props.child("3", Dynamic).run("div", |props| {
									props.set_attribute("x-show", "open");
									props.set_attribute("x-transition:enter", "transition ease-out duration-100");
									props.set_attribute("x-transition:enter-start", "transform opacity-0 scale-95");
									props.set_attribute("x-transition:enter-end", "transform opacity-100 scale-100");
									props.set_attribute("x-transition:leave", "transition ease-in duration-75");
									props.set_attribute("x-transition:leave-start", "transform opacity-100 scale-100");
									props.set_attribute("x-transition:leave-end", "transform opacity-0 scale-95");
									props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("right-0"), NoStyle::Noop("z-10"), Style::MarginTop(8), Style::Width(192), NoStyle::Noop("origin-top-right"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingY(4), NoStyle::Noop("shadow-lg"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-black"), NoStyle::Noop("ring-opacity-5"), Action::Hover(&[NoStyle::Noop("outline-none")])]);
									props.set_attribute("x-ref", "menu-items");
									props.set_attribute("x-description", "Dropdown menu, show/hide based on menu state.");
									props.set_attribute("x-bind:aria-activedescendant", "activeDescendant");
									props.set_attribute("role", "menu");
									props.set_attribute("aria-orientation", "vertical");
									props.set_attribute("aria-labelledby", "user-menu-button");
									props.set_attribute("tabindex", "-1");
									props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
									props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
									props.set_attribute("@keydown.tab", "open = false");
									props.set_attribute("@keydown.enter.prevent", "open = false; focusButton()");
									props.set_attribute("@keyup.space.prevent", "open = false; focusButton()");
									
									props.child("1", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
										props.set_attribute("x-state:on", "Active");
										props.set_attribute("x-state:off", "Not Active");
										props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 0 }");
										props.set_attribute("role", "menuitem");
										props.set_attribute("tabindex", "-1");
										props.set_attribute("id", "user-menu-item-0");
										props.set_attribute("@mouseenter", "onMouseEnter($event)");
										props.set_attribute("@mousemove", "onMouseMove($event, 0)");
										props.set_attribute("@mouseleave", "onMouseLeave($event)");
										props.set_attribute("@click", "open = false; focusButton()");
										
										props.child("0", Label).run(|props| props.set_text("Your Profile"));
									});
									props.child("3", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
										props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 1 }");
										props.set_attribute("role", "menuitem");
										props.set_attribute("tabindex", "-1");
										props.set_attribute("id", "user-menu-item-1");
										props.set_attribute("@mouseenter", "onMouseEnter($event)");
										props.set_attribute("@mousemove", "onMouseMove($event, 1)");
										props.set_attribute("@mouseleave", "onMouseLeave($event)");
										props.set_attribute("@click", "open = false; focusButton()");
										
										props.child("0", Label).run(|props| props.set_text("Settings"));
									});
									props.child("5", Dynamic).run("a", |props| {
										props.set_attribute("href", "#");
										props.styles(&[Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
										props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 2 }");
										props.set_attribute("role", "menuitem");
										props.set_attribute("tabindex", "-1");
										props.set_attribute("id", "user-menu-item-2");
										props.set_attribute("@mouseenter", "onMouseEnter($event)");
										props.set_attribute("@mousemove", "onMouseMove($event, 2)");
										props.set_attribute("@mouseleave", "onMouseLeave($event)");
										props.set_attribute("@click", "open = false; focusButton()");
										
										props.child("0", Label).run(|props| props.set_text("Sign out"));
									});
								});
							});
						});
					});
				});
				props.child("3", Dynamic).run("div", |props| {
					props.set_attribute("x-description", "Mobile menu, show/hide based on menu state.");
					props.styles(&[Screen::Small(&[NoStyle::Noop("hidden")])]);
					props.set_attribute("id", "mobile-menu");
					props.set_attribute("x-show", "open");
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[Style::SpaceY(4), Style::PaddingX(8), Style::PaddingBottom(12), Style::PaddingTop(8)]);
						
						props.child("1", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::Color(Color::Fg(100)), NoStyle::Noop("text-white"), Style::Block, NoStyle::Noop("rounded-md"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium")]);
							props.set_attribute("x-state:on", "Current");
							props.set_attribute("x-state:off", "Default");
							props.set_attribute("aria-current", "page");
							props.set_attribute("x-state-description", r#"Current: "bg-gray-900 text-white", Default: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
							
							props.child("0", Label).run(|props| props.set_text("Dashboard"));
						});
						props.child("3", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::TextColor(Color::Fg(33)), Action::Hover(&[Style::Color(Color::Fg(78))]), Action::Hover(&[NoStyle::Noop("text-white")]), Style::Block, NoStyle::Noop("rounded-md"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium")]);
							props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
							
							props.child("0", Label).run(|props| props.set_text("Team"));
						});
						props.child("5", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::TextColor(Color::Fg(33)), Action::Hover(&[Style::Color(Color::Fg(78))]), Action::Hover(&[NoStyle::Noop("text-white")]), Style::Block, NoStyle::Noop("rounded-md"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium")]);
							props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
							
							props.child("0", Label).run(|props| props.set_text("Projects"));
						});
						props.child("7", Dynamic).run("a", |props| {
							props.set_attribute("href", "#");
							props.styles(&[Style::TextColor(Color::Fg(33)), Action::Hover(&[Style::Color(Color::Fg(78))]), Action::Hover(&[NoStyle::Noop("text-white")]), Style::Block, NoStyle::Noop("rounded-md"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-base"), NoStyle::Noop("font-medium")]);
							props.set_attribute("x-state-description", r#"undefined: "bg-gray-900 text-white", undefined: "text-gray-300 hover:bg-gray-700 hover:text-white""#);
							
							props.child("0", Label).run(|props| props.set_text("Calendar"));
						});
					});
				});
			});
		});
	}
}
